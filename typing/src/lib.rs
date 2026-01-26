use cst::{self, AsCharacter as _, AsIdentifier as _, AsNumber as _};
use errors::{Errors, ResultErrors as _};
use location::{GetSpan as _, SetSpan as _};
use tir::{Typed, env::Env};

#[derive(Debug, Default)]
pub struct Typer {
    id_builder: tir::IdentifierBuilder,
    env: Env,
}

type Result<T, E = Errors> = std::result::Result<T, E>;

impl Typer {
    /// get environment
    pub fn env(&self) -> &Env {
        &self.env
    }

    /// convert constant
    fn constant(&self, constant: &cst::Constant) -> tir::Constant {
        match constant {
            cst::Constant::Number(n) => tir::Constant::nat(n.as_number().clone()),
            cst::Constant::Character(c) => tir::Constant::character(c.as_character()),
            cst::Constant::Builtin(cst::Meta { value: builtin, .. }) => {
                use cst::constant::BuiltinT;
                match builtin {
                    BuiltinT::True => tir::Constant::boolean(true),
                    BuiltinT::False => tir::Constant::boolean(false),
                }
            }
        }
        .with_spanned(constant)
    }

    /// convert expression0
    fn expression0(
        &mut self,
        expression: &cst::expression::Expression0,
    ) -> Result<tir::Expression> {
        use cst::expression::Expression0;
        match expression {
            Expression0::Constant(c) => {
                let c_ty = self.constant(c);
                Ok(tir::Expression::Constant(c_ty))
            }
            Expression0::Variable(var) => {
                let span = var.span();
                let id = self.id_builder.get(var.name());
                let var = self.env.get_expr_var(&id, span).map_err(Errors::from)?;
                Ok(tir::Expression::Variable(var))
            }
            Expression0::Paren(expr) => self.expression(expr.inner()),
        }
    }

    /// convert expression1
    fn expression1(
        &mut self,
        expression: &cst::expression::Expression1,
    ) -> Result<tir::Expression> {
        use cst::expression::Expression1;
        match expression {
            Expression1::TypedExpression { expr, ty, .. } => {
                let (expr, ty) = {
                    let expr_res = self.expression0(expr);
                    let ty_res = self.ty(ty);
                    expr_res.combine(ty_res)?
                };
                expr.restrict_ty(ty).map_err(Errors::from)
            }
            Expression1::Expression0(expr) => self.expression0(expr),
        }
    }

    /// convert expression1
    pub fn expression(
        &mut self,
        expression: &cst::expression::Expression,
    ) -> Result<tir::Expression> {
        self.expression1(expression)
            .map(|expr| expr.with_spanned(expression))
    }

    /// convert type
    pub fn ty(&mut self, ty: &cst::Type) -> Result<tir::Type> {
        match ty {
            cst::Type::Builtin(cst::Meta { value: builtin, .. }) => {
                use cst::ty::BuiltinT;
                let builtin = match builtin {
                    BuiltinT::Nat => tir::TypeBuiltin::nat(),
                    BuiltinT::Bool => tir::TypeBuiltin::bool(),
                    BuiltinT::Char => tir::TypeBuiltin::char(),
                };
                Ok(tir::Type::Builtin(builtin))
            }
            cst::Type::Variable(ty_var) => {
                let span = ty_var.span();
                let id = self.id_builder.get(ty_var.name());
                let alias = self.env.get_alias_ty(&id, span).map_err(Errors::from)?;
                Ok(tir::Type::Alias(alias))
            }
        }
        .map(|ty_f| ty_f.with_spanned(ty))
    }

    fn pattern(&mut self, pattern: &cst::Pattern, ty: &tir::Type) -> Result<tir::Pattern> {
        use cst::Pattern;
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
        definition: &cst::ExpressionDefinition,
    ) -> Result<tir::ExpressionDefinition> {
        self.expression(&definition.body)
            .combine(definition.typed_by().map(|ty| self.ty(ty)).transpose())
            .and_then(|(body, ty_opt)| {
                match &ty_opt {
                    Some(ty) => self.pattern(&definition.pattern, ty),
                    None => self.pattern(&definition.pattern, body.ty()),
                }
                .map(|pattern| (pattern, ty_opt, body))
            })
            .and_then(|(pattern, ty_opt, body)| {
                if let Some(ty) = ty_opt {
                    body.restrict_ty(ty).map_err(Errors::from)
                } else {
                    Ok(body)
                }
                .map(|body| tir::ExpressionDefinition::new(pattern, body))
            })
    }

    /// convert definition
    pub fn definition(
        &mut self,
        definition: &cst::ExpressionDefinition,
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
        definition: &cst::TypeDefinition,
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
    pub fn help(&mut self, var: &cst::help::Variable) -> Result<tir::Help> {
        let id = self.id_builder.get(var.name());
        self.env.get_help(&id, var.span()).map_err(Errors::from)
    }
}
