//! `sir` → `tir`: every expression gets its type, every variable its unique
//! identifier (`a__0`).

use crate::{Checked, CheckedResult, Partial};
use errors::{Errors, Message, ResultErrors as _};

mod error;

use ir::location::{Spanned as _, SpannedSet as _};
use ir::tir::{Typed, env::Env};
use ir::{sir, tir};
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

/// types the files one after the other: the environment keeps the
/// definitions of the previous files (stdlib, REPL inputs), by `DefId`
#[derive(Debug, Default, Clone)]
pub struct Typer {
    env: Env,
    /// the identifier of each definition typed, to name its uses; a use of a
    /// definition missing here is a use of one whose typing failed
    identifiers: HashMap<sir::DefId, Rc<tir::Identifier>>,
}

type Result<T, E = Errors> = std::result::Result<T, E>;

impl Typer {
    pub fn new() -> Self {
        Self::default()
    }

    /// get environment
    pub fn env(&self) -> &Env {
        &self.env
    }

    /// type every command of the file: the commands that typed, and every
    /// error; on error, the environment is left as it was before
    pub fn run(&mut self, file: &Checked<sir::File>) -> CheckedResult<tir::File> {
        let before = self.clone();
        let typed: Partial<Vec<_>> = file
            .commands
            .iter()
            .map(|cmd| match self.command(cmd, &file.defs) {
                Ok(kind) => Partial::ok(kind.map(|kind| tir::Command::new(kind, cmd.span()))),
                Err(errs) => Partial::ok(None).with_errors(
                    errs.into_iter()
                        .map(|err| err.with_recovery(Message::text("the command is left out."))),
                ),
            })
            .collect();
        // a failed command always comes with an error: the silent ones use a
        // definition that failed before
        if !typed.is_ok() {
            *self = before;
        }
        typed
            .map(|commands| commands.into_iter().flatten().collect())
            .check()
    }

    /// type one command; `None` for a command that leaves nothing to run
    fn command(
        &mut self,
        cmd: &sir::Command,
        defs: &BTreeMap<sir::DefId, sir::DefInfo>,
    ) -> Result<Option<tir::CommandKind>> {
        Ok(match &cmd.kind {
            sir::CommandKind::ExpressionDefinition(def) => Some(
                tir::CommandKind::ExpressionDefinition(self.definition(def, defs)?),
            ),
            sir::CommandKind::TypeDefinition(def) => {
                self.type_definition(def, defs[&def.id].doc.as_ref())?;
                None
            }
            sir::CommandKind::Eval(expr) => Some(tir::CommandKind::Eval(self.expression(expr)?)),
            sir::CommandKind::TypeOf(expr) => {
                Some(tir::CommandKind::TypeOf(self.expression(expr)?))
            }
            sir::CommandKind::Help(var) => Some(tir::CommandKind::Help(self.help(var)?)),
        })
    }

    /// diagnostics of an error of `tir`; none for a definition missing from
    /// the environment: the lowering resolved every name, so its typing
    /// failed and was already reported
    fn error(&self, err: tir::error::Error) -> Errors {
        match err {
            tir::error::Error::VariableNotFound { .. } => std::iter::empty().collect(),
            err => Errors::from(error::diagnostic(err)),
        }
    }

    /// the identifier of the definition a use refers to; none, silently, for
    /// a definition whose typing failed (already reported)
    fn identifier(&self, var: &sir::Node<sir::DefId>) -> Result<Rc<tir::Identifier>> {
        self.identifiers
            .get(&var.value)
            .cloned()
            .ok_or_else(|| std::iter::empty().collect())
    }

    /// the identifier of the definition `id` of `name`
    fn bind(&mut self, name: &str, id: sir::DefId) -> Rc<tir::Identifier> {
        let identifier = Rc::new(tir::Identifier {
            name: name.to_string(),
            id,
        });
        self.identifiers.insert(id, Rc::clone(&identifier));
        identifier
    }

    /// convert constant
    fn constant(constant: &sir::Constant) -> tir::Constant {
        match constant {
            sir::Constant::Number(n) => tir::Constant::nat(n.value.clone()),
            sir::Constant::Character(c) => tir::Constant::character(c.value),
            sir::Constant::Builtin(builtin) => tir::Constant::boolean(builtin.value.value()),
        }
        .with_spanned(constant)
    }

    /// convert expression
    fn expression(&mut self, expression: &sir::Expression) -> Result<tir::Expression> {
        match &expression.kind {
            sir::ExpressionKind::Constant(c) => Ok(tir::Expression::Constant(Self::constant(c))),
            sir::ExpressionKind::Variable(var) => {
                let id = self.identifier(var)?;
                let var = self
                    .env
                    .expr_var(&id, var.span)
                    .map_err(|err| self.error(err))?;
                Ok(tir::Expression::Variable(var))
            }
            sir::ExpressionKind::Typed { expr, ty } => {
                let (expr, ty) = {
                    let expr_res = self.expression(expr);
                    let ty_res = self.ty(ty);
                    expr_res.combine(ty_res)?
                };
                expr.restrict_ty(ty).map_err(|err| self.error(err))
            }
        }
        .map(|expr| expr.with_spanned(expression))
    }

    /// convert type
    fn ty(&mut self, ty: &sir::Type) -> Result<tir::Type> {
        match ty {
            sir::Type::Builtin(builtin) => Ok(tir::Type::Builtin(builtin.value.into())),
            sir::Type::Variable(ty_var) => {
                let id = self.identifier(ty_var)?;
                let alias = self
                    .env
                    .type_alias(&id, ty_var.span)
                    .map_err(|err| self.error(err))?;
                Ok(tir::Type::Alias(alias))
            }
        }
        .map(|ty_f| ty_f.with_spanned(ty))
    }

    /// the pattern of a definition, named in `defs`
    fn pattern(
        &mut self,
        pattern: &sir::Pattern,
        ty: &tir::Type,
        defs: &BTreeMap<sir::DefId, sir::DefInfo>,
    ) -> tir::Pattern {
        match pattern {
            sir::Pattern::Variable(var) => {
                let id = self.bind(&defs[&var.value].name.value, var.value);
                self.env.add_expr_def(id.clone(), ty.clone(), var.span());
                tir::Pattern::Variable(tir::PatternVar::from(id).with_spanned(var))
            }
        }
    }

    /// type an expression definition and add it to the environment, with its
    /// documentation
    fn definition(
        &mut self,
        def: &sir::ExpressionDefinition,
        defs: &BTreeMap<sir::DefId, sir::DefInfo>,
    ) -> Result<tir::ExpressionDefinition> {
        let (body, ty) = self
            .expression(&def.body)
            .combine(def.ty.as_ref().map(|ty| self.ty(ty)).transpose())?;
        let pattern = self.pattern(&def.pattern, ty.as_ref().unwrap_or(body.ty()), defs);
        let body = match ty {
            Some(ty) => body.restrict_ty(ty).map_err(|err| self.error(err))?,
            None => body,
        };
        let sir::Pattern::Variable(var) = &def.pattern;
        if let Some(doc) = &defs[&var.value].doc {
            for id in pattern.names() {
                self.env.set_doc(id, doc.clone());
            }
        }
        Ok(tir::ExpressionDefinition::new(pattern, body))
    }

    /// type a type definition and add it to the environment, with its
    /// documentation
    fn type_definition(
        &mut self,
        def: &sir::TypeDefinition,
        doc: Option<&tir::Documentation>,
    ) -> Result<()> {
        let ty = self.ty(&def.ty)?;
        let id = self.bind(&def.name.value, def.id);
        self.env.add_type_def(id.clone(), ty, def.name.span());
        if let Some(doc) = doc {
            self.env.set_doc(&id, doc.clone());
        }
        Ok(())
    }

    /// convert help variable
    fn help(&mut self, var: &sir::Node<sir::DefId>) -> Result<tir::Help> {
        let id = self.identifier(var)?;
        self.env.help(&id, var.span).map_err(|err| self.error(err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, lowering::Lowerer, parser::Parser};
    use ir::location::SourceId;

    /// the passes up to typing, on the inputs of one project; an input with
    /// errors is forgotten, as `compile` does
    #[derive(Default)]
    struct Session {
        lowerer: Lowerer,
        typer: Typer,
    }

    impl Session {
        /// type `src`: the type of each command, or the error codes
        fn run(&mut self, src: &str) -> std::result::Result<Vec<String>, Vec<i32>> {
            let (lowerer, typer) = (self.lowerer.clone(), self.typer.clone());
            let typed = self.typed(src);
            if typed.is_err() {
                (self.lowerer, self.typer) = (lowerer, typer);
            }
            typed
                .map(|file| {
                    file.commands()
                        .iter()
                        .map(|cmd| match &cmd.kind {
                            tir::CommandKind::ExpressionDefinition(def) => {
                                format!("{:?}", def.ty())
                            }
                            tir::CommandKind::Eval(expr) | tir::CommandKind::TypeOf(expr) => {
                                format!("{:?}", expr.ty())
                            }
                            tir::CommandKind::Help(_) => "help".to_string(),
                        })
                        .collect()
                })
                .map_err(|errs| errs.into_iter().map(|err| err.code()).collect())
        }

        fn typed(&mut self, src: &str) -> Result<tir::File> {
            let tokens = Lexer::builder().build().run(src)?;
            let pir = Parser::builder()
                .source(SourceId::Repl)
                .build()
                .run(&tokens)?;
            let sir = self.lowerer.run(&pir)?;
            Ok(self.typer.run(&sir)?.into_inner())
        }
    }

    fn type_source(src: &str) -> std::result::Result<Vec<String>, Vec<i32>> {
        Session::default().run(src)
    }

    fn builtin(kind: &str) -> String {
        format!(
            "{:?}",
            tir::Type::Builtin(match kind {
                "nat" => tir::ty::Builtin::nat(),
                "bool" => tir::ty::Builtin::bool(),
                _ => tir::ty::Builtin::char(),
            })
        )
    }

    #[test]
    fn constants_and_variables() {
        assert_eq!(
            type_source("Def a := 3.\nEval a.\nEval 'c'.\nEval __Constant_true__."),
            Ok(vec![
                builtin("nat"),
                builtin("nat"),
                builtin("char"),
                builtin("bool")
            ])
        );
    }

    #[test]
    fn type_definitions_leave_no_command() {
        let types = type_source("Type T := __Type_Char__.\nDef c : T := 'x'.\nEval (c : T).");
        assert_eq!(types.map(|types| types.len()), Ok(2));
    }

    #[test]
    fn every_type_error() {
        assert_eq!(
            type_source("Eval 3 : __Type_Char__.\nDef c : __Type_Bool__ := 'x'."),
            Err(vec![302, 302])
        );
    }

    #[test]
    fn no_cascade_from_a_broken_definition() {
        // only the original errors: the uses of `a` and `b` fail silently
        assert_eq!(
            type_source(
                "Def a : __Type_Char__ := 1.\nEval a.\nDef b := a.\nTypeOf b.\nEval 2 : __Type_Bool__."
            ),
            Err(vec![302, 302])
        );
    }

    #[test]
    fn input_forgotten_after_an_error() {
        let mut session = Session::default();
        assert!(session.run("Def a := 1.\nEval 1 : __Type_Char__.").is_err());
        // `a` was not kept: the input had an error
        assert_eq!(session.run("Eval a."), Err(vec![301]));
        assert!(session.run("Def a := 1.").is_ok());
        assert_eq!(session.run("Eval a."), Ok(vec![builtin("nat")]));
    }

    #[test]
    fn documentation_of_the_definitions() {
        let mut session = Session::default();
        assert!(
            session
                .run("(** the answer *) Def a := 42.\nHelp a.")
                .is_ok()
        );
        let info = session
            .typer
            .env()
            .iter()
            .find(|info| info.id.name() == "a")
            .unwrap();
        assert_eq!(
            info.doc.as_ref().map(|doc| doc.lines().to_vec()),
            Some(vec!["the answer".to_string()])
        );
    }

    #[test]
    fn typed_commands_kept_with_errors() {
        let mut session = Session::default();
        let tokens = Lexer::builder()
            .build()
            .run("Eval 1.\nEval 1 : __Type_Char__.\nTypeOf 'c'.");
        let pir = Parser::builder()
            .build()
            .run(&tokens.ok().unwrap())
            .ok()
            .unwrap();
        let sir = session.lowerer.run(&pir).ok().unwrap();
        let typed = Partial::from(session.typer.run(&sir));
        assert_eq!(typed.errors.len(), 1);
        assert_eq!(typed.value.commands().len(), 2);
        let recovery = (&typed.errors)
            .into_iter()
            .next()
            .and_then(|err| err.recovery());
        assert_eq!(
            recovery.map(|msg| msg.make_string(&pp::MessageTheme::default())),
            Some("the command is left out.".to_string())
        );
    }
}
