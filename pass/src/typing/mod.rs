//! `sir` → `tir`: every expression gets its type, every variable its unique
//! identifier (`a__0`).

use crate::Partial;
use errors::{Errors, Message, ResultErrors as _};

mod error;

use ir::location::{Spanned as _, SpannedSet as _};
use ir::tir::{Typed, env::Env};
use ir::{sir, tir};
use std::collections::HashSet;

/// options of `Set` / `Unset` (no effect for now)
const OPTIONS: [&str; 3] = ["DebugLexer", "DebugParser", "DebugTyper"];

/// types the files one after the other: the environment keeps the
/// definitions of the previous files (stdlib, REPL inputs)
#[derive(Debug, Default, Clone)]
pub struct Typer {
    id_builder: tir::IdentifierBuilder,
    env: Env,
    /// names whose definition failed in the current file: their uses fail
    /// silently, only the original error is shown
    broken: HashSet<String>,
}

type Result<T, E = Errors> = std::result::Result<T, E>;

impl Typer {
    pub fn builder() -> TyperBuilder {
        TyperBuilder
    }

    /// get environment
    pub fn env(&self) -> &Env {
        &self.env
    }

    /// type every command of `file`: the commands that typed, and every
    /// error; on error, the environment is left as it was before the file
    pub fn run(&mut self, file: &sir::File) -> Partial<tir::File> {
        let before = self.clone();
        self.broken.clear();
        let typed: Partial<Vec<_>> =
            file.commands()
                .iter()
                .map(|cmd| match self.command(cmd, file.defs()) {
                    Ok(kind) => Partial::ok(kind.map(|kind| tir::Command::new(kind, cmd.span()))),
                    Err(errs) => {
                        self.mark_broken(cmd);
                        Partial::ok(None).with_errors(errs.into_iter().map(|err| {
                            err.with_recovery(Message::text("the command is left out."))
                        }))
                    }
                })
                .collect();
        // a failed command always comes with an error: the silent ones follow a broken one
        if !typed.is_ok() {
            *self = before;
        }
        typed.map(|commands| commands.into_iter().flatten().collect())
    }

    /// type one command; `None` for a command that leaves nothing to run
    fn command(
        &mut self,
        cmd: &sir::Command,
        defs: &sir::DefTable,
    ) -> Result<Option<tir::CommandKind>> {
        Ok(match &cmd.kind {
            sir::CommandKind::ExpressionDefinition(def) => Some(
                tir::CommandKind::ExpressionDefinition(self.definition(def, defs.doc(def.id))?),
            ),
            sir::CommandKind::TypeDefinition(def) => {
                self.type_definition(def, defs.doc(def.id))?;
                None
            }
            sir::CommandKind::Eval(expr) => Some(tir::CommandKind::Eval(self.expression(expr)?)),
            sir::CommandKind::TypeOf(expr) => {
                Some(tir::CommandKind::TypeOf(self.expression(expr)?))
            }
            sir::CommandKind::Help(var) => Some(tir::CommandKind::Help(self.help(var)?)),
            sir::CommandKind::Set(option) | sir::CommandKind::UnSet(option) => {
                if !OPTIONS.contains(&option.name()) {
                    return Err(error::unknown_option(option).into());
                }
                None
            }
        })
    }

    /// the names defined by a failed command are broken
    fn mark_broken(&mut self, cmd: &sir::Command) {
        match &cmd.kind {
            sir::CommandKind::ExpressionDefinition(def) => {
                let sir::Pattern::Variable(var) = &def.pattern;
                self.broken.insert(var.value.clone());
            }
            sir::CommandKind::TypeDefinition(def) => {
                self.broken.insert(def.name.value.clone());
            }
            _ => {}
        }
    }

    /// diagnostics of an error of `tir`; none for a use of a broken name
    fn error(&self, err: tir::error::Error) -> Errors {
        match &err {
            tir::error::Error::VariableNotFound { identifier, .. }
                if self.broken.contains(identifier.name()) =>
            {
                std::iter::empty().collect()
            }
            _ => Errors::from(error::diagnostic(err)),
        }
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
                let id = self.id_builder.get(var.name());
                let var = self
                    .env
                    .expr_var(&id, var.span())
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
                let id = self.id_builder.get(ty_var.name());
                let alias = self
                    .env
                    .type_alias(&id, ty_var.span())
                    .map_err(|err| self.error(err))?;
                Ok(tir::Type::Alias(alias))
            }
        }
        .map(|ty_f| ty_f.with_spanned(ty))
    }

    fn pattern(&mut self, pattern: &sir::Pattern, ty: &tir::Type) -> tir::Pattern {
        match pattern {
            sir::Pattern::Variable(var) => {
                let id = self.id_builder.build(var.name());
                self.env.add_expr_def(id.clone(), ty.clone(), var.span());
                tir::Pattern::Variable(tir::PatternVar::from(id).with_spanned(var))
            }
        }
    }

    /// type an expression definition and add it to the environment, with the
    /// documentation `doc` (from the `DefTable` of its file)
    fn definition(
        &mut self,
        def: &sir::ExpressionDefinition,
        doc: Option<&tir::Documentation>,
    ) -> Result<tir::ExpressionDefinition> {
        let (body, ty) = self
            .expression(&def.body)
            .combine(def.ty.as_ref().map(|ty| self.ty(ty)).transpose())?;
        let pattern = self.pattern(&def.pattern, ty.as_ref().unwrap_or(body.ty()));
        let body = match ty {
            Some(ty) => body.restrict_ty(ty).map_err(|err| self.error(err))?,
            None => body,
        };
        if let Some(doc) = doc {
            for id in pattern.names() {
                self.env.set_doc(id, doc.clone());
            }
        }
        Ok(tir::ExpressionDefinition::new(pattern, body))
    }

    /// type a type definition and add it to the environment, with the
    /// documentation `doc`
    fn type_definition(
        &mut self,
        def: &sir::TypeDefinition,
        doc: Option<&tir::Documentation>,
    ) -> Result<()> {
        let ty = self.ty(&def.ty)?;
        let id = self.id_builder.build(def.name.name());
        self.env.add_type_def(id.clone(), ty, def.name.span());
        if let Some(doc) = doc {
            self.env.set_doc(&id, doc.clone());
        }
        Ok(())
    }

    /// convert help variable
    fn help(&mut self, var: &sir::Variable) -> Result<tir::Help> {
        let id = self.id_builder.get(var.name());
        self.env
            .help(&id, var.span())
            .map_err(|err| self.error(err))
    }
}

/// arguments of a `Typer` (none yet)
#[derive(Debug, Default, Clone, Copy)]
pub struct TyperBuilder;

impl TyperBuilder {
    pub fn build(self) -> Typer {
        Typer::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, lowering::Lowering, parser::Parser};
    use ir::location::SourceId;

    /// lex, parse and lower `src`
    fn sir(src: &str) -> sir::File {
        let tokens = Lexer::builder()
            .build()
            .run(src)
            .into_result()
            .unwrap_or_else(|_| panic!("lex {src}"));
        let pir = Parser::builder()
            .build()
            .run(tokens)
            .into_result()
            .unwrap_or_else(|_| panic!("parse {src}"));
        Lowering::builder()
            .source(SourceId::Repl)
            .build()
            .run(&pir)
            .value
    }

    /// type `src` with `typer`: the type of each command, or the error codes
    fn type_with(typer: &mut Typer, src: &str) -> std::result::Result<Vec<String>, Vec<i32>> {
        typer
            .run(&sir(src))
            .into_result()
            .map(|file| {
                file.commands()
                    .iter()
                    .map(|cmd| match &cmd.kind {
                        tir::CommandKind::ExpressionDefinition(def) => format!("{:?}", def.ty()),
                        tir::CommandKind::Eval(expr) | tir::CommandKind::TypeOf(expr) => {
                            format!("{:?}", expr.ty())
                        }
                        tir::CommandKind::Help(_) => "help".to_string(),
                    })
                    .collect()
            })
            .map_err(|errs| errs.into_iter().map(|err| err.code()).collect())
    }

    fn type_source(src: &str) -> std::result::Result<Vec<String>, Vec<i32>> {
        type_with(&mut Typer::builder().build(), src)
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
    fn every_error_with_its_code() {
        assert_eq!(
            type_source("Eval x.\nEval 3 : __Type_Char__.\nType T := U.\nHelp y.\nSet Foo."),
            Err(vec![301, 302, 301, 301, 103])
        );
    }

    #[test]
    fn no_cascade_from_a_broken_definition() {
        // only the original error: the uses of `a` and `b` fail silently
        assert_eq!(
            type_source("Def a := x.\nEval a.\nDef b := a.\nTypeOf b.\nEval y."),
            Err(vec![301, 301])
        );
    }

    #[test]
    fn known_options_are_accepted() {
        assert_eq!(
            type_source("Set DebugLexer.\nUnset DebugTyper."),
            Ok(vec![])
        );
    }

    #[test]
    fn environment_unchanged_after_an_error() {
        let mut typer = Typer::builder().build();
        assert!(type_with(&mut typer, "Def a := 1.\nEval x.").is_err());
        // `a` was not kept: the file had an error
        assert_eq!(type_with(&mut typer, "Eval a."), Err(vec![301]));
        assert!(type_with(&mut typer, "Def a := 1.").is_ok());
        assert_eq!(type_with(&mut typer, "Eval a."), Ok(vec![builtin("nat")]));
    }

    #[test]
    fn documentation_from_the_def_table() {
        let mut typer = Typer::builder().build();
        assert!(type_with(&mut typer, "(** the answer *) Def a := 42.\nHelp a.").is_ok());
        let info = typer
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
        let typed = Typer::builder()
            .build()
            .run(&sir("Eval 1.\nEval x.\nTypeOf 'c'."));
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
