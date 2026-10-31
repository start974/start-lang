//! `sir` → `tir`: every expression gets its type, every variable its unique
//! identifier (`a__0`).

use errors::{Errors, ResultErrors as _};

mod error;

use ir::location::{Spanned as _, SpannedSet as _};
use ir::tir::{Typed, env::Env};
use ir::{sir, tir};

#[derive(Debug, Default)]
pub struct Typer {
    id_builder: tir::IdentifierBuilder,
    env: Env,
}

type Result<T, E = Errors> = std::result::Result<T, E>;

/// diagnostics of an error of `tir`
fn error(err: ir::tir::error::Error) -> Errors {
    Errors::from(error::diagnostic(err))
}

impl Typer {
    /// get environment
    pub fn env(&self) -> &Env {
        &self.env
    }

    /// convert constant
    fn constant(&self, constant: &sir::Constant) -> tir::Constant {
        match constant {
            sir::Constant::Number(n) => tir::Constant::nat(n.value.clone()),
            sir::Constant::Character(c) => tir::Constant::character(c.value),
            sir::Constant::Builtin(builtin) => tir::Constant::boolean(builtin.value.value()),
        }
        .with_spanned(constant)
    }

    /// convert expression
    pub fn expression(&mut self, expression: &sir::Expression) -> Result<tir::Expression> {
        match &expression.kind {
            sir::ExpressionKind::Constant(c) => Ok(tir::Expression::Constant(self.constant(c))),
            sir::ExpressionKind::Variable(var) => {
                let id = self.id_builder.get(var.name());
                let var = self.env.get_expr_var(&id, var.span()).map_err(error)?;
                Ok(tir::Expression::Variable(var))
            }
            sir::ExpressionKind::Typed { expr, ty } => {
                let (expr, ty) = {
                    let expr_res = self.expression(expr);
                    let ty_res = self.ty(ty);
                    expr_res.combine(ty_res)?
                };
                expr.restrict_ty(ty).map_err(error)
            }
        }
        .map(|expr| expr.with_spanned(expression))
    }

    /// convert type
    pub fn ty(&mut self, ty: &sir::Type) -> Result<tir::Type> {
        match ty {
            sir::Type::Builtin(builtin) => Ok(tir::Type::Builtin(builtin.value.into())),
            sir::Type::Variable(ty_var) => {
                let id = self.id_builder.get(ty_var.name());
                let alias = self.env.get_alias_ty(&id, ty_var.span()).map_err(error)?;
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
    pub fn definition(
        &mut self,
        def: &sir::ExpressionDefinition,
        doc: Option<&tir::Documentation>,
    ) -> Result<tir::ExpressionDefinition> {
        let (body, ty) = self
            .expression(&def.body)
            .combine(def.ty.as_ref().map(|ty| self.ty(ty)).transpose())?;
        let pattern = self.pattern(&def.pattern, ty.as_ref().unwrap_or(body.ty()));
        let body = match ty {
            Some(ty) => body.restrict_ty(ty).map_err(error)?,
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
    pub fn type_definition(
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
    pub fn help(&mut self, var: &sir::Variable) -> Result<tir::Help> {
        let id = self.id_builder.get(var.name());
        self.env.get_help(&id, var.span()).map_err(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lowering::Lowering;
    use ir::location::SourceId;

    /// lex, parse and lower `src` (one command per line), then type every
    /// command with one `Typer`; the result of each command, in order
    fn type_source(src: &str) -> (Typer, Vec<std::result::Result<String, Vec<i32>>>) {
        let mut lowering = Lowering::new(SourceId::Repl);
        let mut typer = Typer::default();
        let mut results = Vec::new();
        let mut offset = 0;
        for line in src.lines() {
            let tokens = crate::lexer::lex(line, offset).unwrap_or_else(|_| panic!("lex {line}"));
            let Ok(crate::parser::CommandOrEnd::Command(cmd)) = crate::parser::parse(tokens) else {
                panic!("parse {line}")
            };
            let cmd = lowering.command(&cmd).clone();
            let defs = lowering.defs();
            let result = match &cmd.kind {
                sir::CommandKind::ExpressionDefinition(def) => typer
                    .definition(def, defs.doc(def.id))
                    .map(|def| format!("{:?}", def.ty())),
                sir::CommandKind::TypeDefinition(def) => typer
                    .type_definition(def, defs.doc(def.id))
                    .map(|()| "type".to_string()),
                sir::CommandKind::Eval(expr) | sir::CommandKind::TypeOf(expr) => typer
                    .expression(expr)
                    .map(|expr| format!("{:?}", expr.ty())),
                sir::CommandKind::Help(var) => typer.help(var).map(|_| "help".to_string()),
                _ => Ok("option".to_string()),
            };
            results.push(result.map_err(|errs| errs.into_iter().map(|e| e.code()).collect()));
            offset += line.len() + 1;
        }
        (typer, results)
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
        let (_, results) = type_source("Def a := 3.\nEval a.\nEval 'c'.\nEval __Constant_true__.");
        assert_eq!(
            results,
            [
                Ok(builtin("nat")),
                Ok(builtin("nat")),
                Ok(builtin("char")),
                Ok(builtin("bool"))
            ]
        );
    }

    #[test]
    fn alias_and_restriction() {
        let (_, results) =
            type_source("Type T := __Type_Char__.\nDef c : T := 'x'.\nEval (c : T).");
        assert_eq!(results[0], Ok("type".to_string()));
        assert!(results[1].is_ok() && results[2].is_ok());
    }

    #[test]
    fn errors_have_their_codes() {
        let (_, results) = type_source("Eval x.\nEval 3 : __Type_Char__.\nType T := U.\nHelp y.");
        assert_eq!(
            results,
            [
                Err(vec![301]),
                Err(vec![302]),
                Err(vec![301]),
                Err(vec![301])
            ]
        );
    }

    #[test]
    fn documentation_from_the_def_table() {
        let (typer, results) = type_source("(** the answer *) Def a := 42.\nHelp a.");
        assert_eq!(results[1], Ok("help".to_string()));
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
}
