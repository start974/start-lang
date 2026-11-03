//! `sir` → `tir`: every expression gets its type, every variable its unique
//! identifier (`a@0`).

use crate::{Checked, CheckedResult, Partial};
use errors::{Errors, Message, ResultErrors as _};

mod error;

use ir::location::{Span, Spanned as _};
use ir::{sir, tir};
use std::collections::{BTreeMap, HashMap};

/// definitions of a file, from the lowering
type Defs = BTreeMap<sir::DefId, sir::DefInfo>;

/// types the files one after the other: the definitions of the previous
/// files (stdlib, REPL inputs) are kept, by `DefId`
#[derive(Debug, Default, Clone)]
pub struct Typer {
    /// every definition typed; a use of a definition missing here is a use of
    /// one whose typing failed
    defs: HashMap<sir::DefId, tir::DefInfo>,
    /// print the `tir`; set again by `Stdlib::debug` once the stdlib is loaded
    pub(crate) debug: bool,
}

/// arguments of a `Typer`
#[derive(Debug, Default, Clone, Copy)]
pub struct TyperBuilder {
    debug: bool,
}

impl TyperBuilder {
    /// print the `tir`
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    pub fn build(self) -> Typer {
        Typer {
            debug: self.debug,
            ..Typer::default()
        }
    }
}

type Result<T, E = Errors> = std::result::Result<T, E>;

impl Typer {
    pub fn builder() -> TyperBuilder {
        TyperBuilder::default()
    }

    /// every definition typed, in no order
    pub fn defs(&self) -> impl Iterator<Item = &tir::DefInfo> {
        self.defs.values()
    }

    /// type every command of the file: the commands that typed, and every
    /// error; going back to the state before a file with errors is up to the
    /// caller (`Stdlib::run`), as for the lowerer
    pub fn run(&mut self, file: &Checked<sir::File>) -> CheckedResult<tir::File> {
        let typed: Partial<Vec<_>> = file
            .commands
            .iter()
            .map(|cmd| match self.command(cmd, &file.defs) {
                Ok(kind) => Partial::ok(kind.map(|kind| tir::Command {
                    kind,
                    span: cmd.span,
                })),
                Err(errs) => Partial::ok(None).with_errors(
                    errs.into_iter()
                        .map(|err| err.with_recovery(Message::text("the command is left out"))),
                ),
            })
            .collect();
        typed
            .map(|commands| tir::File {
                commands: commands.into_iter().flatten().collect(),
            })
            .check_debug(self.debug)
    }

    /// type one command; `None` for a command that leaves nothing to run
    fn command(&mut self, cmd: &sir::Command, defs: &Defs) -> Result<Option<tir::CommandKind>> {
        Ok(match &cmd.kind {
            sir::CommandKind::ExpressionDefinition(def) => Some(
                tir::CommandKind::ExpressionDefinition(Box::new(self.definition(def, defs)?)),
            ),
            sir::CommandKind::TypeDefinition(def) => {
                let ty = self.ty(&def.ty)?;
                self.bind(def.id, ty, defs);
                None
            }
            sir::CommandKind::Eval(expr) => Some(tir::CommandKind::Eval(self.expression(expr)?)),
            sir::CommandKind::TypeOf(expr) => {
                Some(tir::CommandKind::TypeOf(self.expression(expr)?))
            }
            sir::CommandKind::Help(var) => Some(tir::CommandKind::Help(tir::Help {
                def: self.def(var)?.clone(),
                span: var.span,
            })),
        })
    }

    /// the definition a use refers to; none, silently, for a definition whose
    /// typing failed (already reported): the lowering resolved every name
    fn def(&self, var: &sir::Node<sir::Identifier>) -> Result<&tir::DefInfo> {
        self.defs
            .get(&var.value.id)
            .ok_or_else(|| std::iter::empty().collect())
    }

    /// add the definition `id` of `defs`, of type `ty`, with its documentation
    fn bind(&mut self, id: sir::DefId, ty: tir::Type, defs: &Defs) -> tir::Identifier {
        let info = &defs[&id];
        let identifier = tir::Identifier {
            name: info.name.value.clone(),
            id,
        };
        self.defs.insert(
            id,
            tir::DefInfo {
                id: identifier.clone(),
                kind: info.kind,
                ty,
                doc: info.doc.clone(),
            },
        );
        identifier
    }

    /// `expr` restricted to `ty`, written at `span`
    fn restrict(expr: tir::Expression, ty: tir::Type, span: Span) -> Result<tir::Expression> {
        if expr.ty.is_compatible(&ty) {
            Ok(tir::Expression { ty, ..expr })
        } else {
            Err(error::unexpected_type(&ty, &expr.ty, span).into())
        }
    }

    /// convert constant
    fn constant(constant: &sir::Constant) -> tir::Constant {
        match constant {
            sir::Constant::Number(n) => tir::Constant::Nat(n.value.clone()),
            sir::Constant::Character(c) => tir::Constant::Char(c.value),
            sir::Constant::Builtin(builtin) => tir::Constant::Bool(builtin.value.value()),
        }
    }

    /// convert expression
    fn expression(&self, expression: &sir::Expression) -> Result<tir::Expression> {
        let (kind, ty) = match &expression.kind {
            sir::ExpressionKind::Constant(c) => {
                let c = Self::constant(c);
                let ty = c.ty();
                (tir::ExpressionKind::Constant(c), ty)
            }
            sir::ExpressionKind::Variable(var) => {
                let def = self.def(var)?;
                let var = tir::Node {
                    value: def.id.clone(),
                    span: var.span,
                };
                (tir::ExpressionKind::Variable(var), def.ty.clone())
            }
            sir::ExpressionKind::Typed { expr, ty: sir_ty } => {
                let (expr, ty) = self.expression(expr).combine(self.ty(sir_ty))?;
                let expr = Self::restrict(expr, ty, sir_ty.span())?;
                (expr.kind, expr.ty)
            }
        };
        Ok(tir::Expression {
            kind,
            ty,
            span: expression.span,
        })
    }

    /// convert type
    fn ty(&self, ty: &sir::Type) -> Result<tir::Type> {
        match ty {
            sir::Type::Builtin(builtin) => Ok(tir::Type::Builtin(builtin.value)),
            sir::Type::Variable(var) => {
                let def = self.def(var)?;
                Ok(tir::Type::Alias {
                    name: def.id.clone(),
                    ty: Box::new(def.ty.clone()),
                })
            }
        }
    }

    /// type an expression definition and add it, with its documentation;
    /// a definition that fails is not added
    fn definition(
        &mut self,
        def: &sir::ExpressionDefinition,
        defs: &Defs,
    ) -> Result<tir::ExpressionDefinition> {
        let (body, ty) = self
            .expression(&def.body)
            .combine(def.ty.as_ref().map(|ty| self.ty(ty)).transpose())?;
        let body = match ty.zip(def.ty.as_ref()) {
            Some((ty, sir_ty)) => Self::restrict(body, ty, sir_ty.span())?,
            None => body,
        };
        let sir::Pattern::Variable(var) = &def.pattern;
        let id = self.bind(var.value.id, body.ty.clone(), defs);
        Ok(tir::ExpressionDefinition {
            pattern: tir::Pattern::Variable(tir::Node {
                value: id,
                span: var.span,
            }),
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, lowering::Lowerer, parser::Parser};
    use ir::location::SourceId;
    use ir::tir::ty::BuiltinType;

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
                    file.commands
                        .iter()
                        .map(|cmd| match &cmd.kind {
                            tir::CommandKind::ExpressionDefinition(def) => {
                                format!("{:?}", def.body.ty)
                            }
                            tir::CommandKind::Eval(expr) | tir::CommandKind::TypeOf(expr) => {
                                format!("{:?}", expr.ty)
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

    fn builtin(kind: BuiltinType) -> String {
        format!("{:?}", tir::Type::Builtin(kind))
    }

    #[test]
    fn constants_and_variables() {
        assert_eq!(
            type_source("Def a := 3.\nEval a.\nEval 'c'.\nEval __Constant_true__."),
            Ok(vec![
                builtin(BuiltinType::Nat),
                builtin(BuiltinType::Nat),
                builtin(BuiltinType::Char),
                builtin(BuiltinType::Bool)
            ])
        );
    }

    #[test]
    fn type_definitions_leave_no_command() {
        let types = type_source("Type T := __Type_Char__.\nDef c : T := 'x'.\nEval (c : T).");
        assert_eq!(types.map(|types| types.len()), Ok(2));
    }

    #[test]
    fn restriction_keeps_the_alias_and_the_span() {
        let mut session = Session::default();
        let file = session
            .typed("Type T := __Type_Char__.\nEval ('x' : T).")
            .ok()
            .unwrap();
        let tir::CommandKind::Eval(expr) = &file.commands[0].kind else {
            panic!("an Eval");
        };
        assert!(matches!(&expr.ty, tir::Type::Alias { name, .. } if name.name == "T"));
        assert_eq!(expr.ty.builtin(), BuiltinType::Char);
        // parentheses included
        assert_eq!((expr.span.start(), expr.span.end()), (30, 39));
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
        assert_eq!(session.run("Eval a."), Ok(vec![builtin(BuiltinType::Nat)]));
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
            .defs()
            .find(|info| info.id.name == "a")
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
        assert_eq!(typed.value.commands.len(), 2);
        let recovery = (&typed.errors)
            .into_iter()
            .next()
            .and_then(|err| err.recovery());
        assert_eq!(
            recovery.map(|msg| msg.make_string(&pp::MessageTheme::default())),
            Some("the command is left out".to_string())
        );
    }
}
