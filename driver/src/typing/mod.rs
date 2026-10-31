use errors::{Errors, ResultErrors as _};

mod error;

use ir::location::{Spanned as _, SpannedSet as _};
use ir::tir::{Typed, env::Env};
use ir::{pir, tir};

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
    fn constant(&self, constant: &pir::Constant) -> tir::Constant {
        match constant {
            pir::Constant::Number(n) => tir::Constant::nat(n.value.clone()),
            pir::Constant::Character(c) => tir::Constant::character(c.value),
            pir::Constant::Builtin(builtin) => tir::Constant::boolean(builtin.value.value()),
        }
        .with_spanned(constant)
    }

    /// convert expression
    pub fn expression(&mut self, expression: &pir::Expression) -> Result<tir::Expression> {
        match expression {
            pir::Expression::Constant(c) => {
                let c_ty = self.constant(c);
                Ok(tir::Expression::Constant(c_ty))
            }
            pir::Expression::Variable(var) => {
                let span = var.span();
                let id = self.id_builder.get(var.name());
                let var = self.env.get_expr_var(&id, span).map_err(error)?;
                Ok(tir::Expression::Variable(var))
            }
            pir::Expression::Paren(expr) => self.expression(expr.inner()),
            pir::Expression::Typed { expr, ty, .. } => {
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
    pub fn ty(&mut self, ty: &pir::Type) -> Result<tir::Type> {
        match ty {
            pir::Type::Builtin(builtin) => Ok(tir::Type::Builtin(builtin.value.into())),
            pir::Type::Variable(ty_var) => {
                let span = ty_var.span();
                let id = self.id_builder.get(ty_var.name());
                let alias = self.env.get_alias_ty(&id, span).map_err(error)?;
                Ok(tir::Type::Alias(alias))
            }
        }
        .map(|ty_f| ty_f.with_spanned(ty))
    }

    fn pattern(&mut self, pattern: &pir::Pattern, ty: &tir::Type) -> Result<tir::Pattern> {
        use pir::Pattern;
        match pattern {
            Pattern::Variable(var) => {
                let id = self.id_builder.build(var.name());
                self.env.add_expr_def(id.clone(), ty.clone(), var.span());
                let pattern_var = tir::PatternVar::from(id).with_spanned(var);
                Ok(tir::Pattern::Variable(pattern_var))
            }
        }
    }

    /// type expression definition
    fn expression_definition(
        &mut self,
        definition: &pir::ExpressionDefinition,
    ) -> Result<tir::ExpressionDefinition> {
        self.expression(&definition.body)
            .combine(
                definition
                    .typed_by
                    .as_ref()
                    .map(|typed_by| self.ty(&typed_by.ty))
                    .transpose(),
            )
            .and_then(|(body, ty_opt)| {
                match &ty_opt {
                    Some(ty) => self.pattern(&definition.pattern, ty),
                    None => self.pattern(&definition.pattern, body.ty()),
                }
                .map(|pattern| (pattern, ty_opt, body))
            })
            .and_then(|(pattern, ty_opt, body)| {
                if let Some(ty) = ty_opt {
                    body.restrict_ty(ty).map_err(error)
                } else {
                    Ok(body)
                }
                .map(|body| tir::ExpressionDefinition::new(pattern, body))
            })
    }

    /// convert definition
    pub fn definition(
        &mut self,
        definition: &pir::ExpressionDefinition,
        doc_opt: Option<tir::Documentation>,
    ) -> Result<tir::ExpressionDefinition> {
        let expr_def = self.expression_definition(definition)?;
        if let Some(doc) = doc_opt {
            for id in expr_def.pattern().names() {
                self.env.set_doc(id, doc.clone());
            }
        }
        Ok(expr_def)
    }

    /// add type definition
    pub fn type_definition(
        &mut self,
        definition: &pir::TypeDefinition,
        doc: Option<tir::Documentation>,
    ) -> Result<()> {
        let id = self.id_builder.build(definition.name.name());
        let ty = self.ty(&definition.ty)?;
        self.env
            .add_type_def(id.clone(), ty.clone(), definition.name.span());
        if let Some(doc) = doc {
            self.env.set_doc(&id, doc.clone());
        }
        Ok(())
    }

    /// convert help variable
    pub fn help(&mut self, var: &pir::Variable) -> Result<tir::Help> {
        let id = self.id_builder.get(var.name());
        self.env.get_help(&id, var.span()).map_err(error)
    }
}
