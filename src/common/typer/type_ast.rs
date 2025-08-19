use super::ast::{self, Typed as _};
use super::env::Env;
use super::error::Error;
use crate::lexer::Meta;
use crate::parser::cst::{self, AsCharacter as _, AsIdentifier as _, AsNumber as _};
use crate::utils::error::ResultExt as _;
use crate::utils::location::{Located as _, LocatedSet};

#[derive(Debug, Default)]
pub struct Typer {
    id_builder: ast::IdentifierBuilder,
    env: Env,
}

type Result<T, E = Vec<Error>> = std::result::Result<T, E>;

fn to_errs(err: Error) -> Vec<Error> {
    vec![err]
}

impl Typer {
    /// convert constant
    fn constant(&self, constant: &cst::Constant) -> ast::Constant {
        match constant {
            cst::Constant::Number(n) => ast::Constant::nat(n.as_number().clone()),
            cst::Constant::Character(c) => ast::Constant::character(c.as_character()),
            cst::Constant::Builtin(Meta { value: builtin, .. }) => {
                use cst::constant::BuiltinT;
                match builtin {
                    BuiltinT::True => ast::Constant::boolean(true),
                    BuiltinT::False => ast::Constant::boolean(false),
                }
            }
        }
        .with_loc(constant)
    }

    /// convert expression0
    fn expression0(
        &mut self,
        expression: &cst::expression::Expression0,
    ) -> Result<ast::Expression> {
        use cst::expression::Expression0;
        match expression {
            Expression0::Constant(c) => {
                let c_ty = self.constant(c);
                Ok(ast::Expression::Constant(c_ty))
            }
            Expression0::Variable(var) => {
                let loc = var.loc();
                let id = self.id_builder.get(var.name());
                let var = self
                    .env
                    .get_expr_var(&id, loc.clone())
                    .map_err(Error::from)
                    .map_err(to_errs)?;
                Ok(ast::Expression::Variable(var))
            }
            Expression0::Paren(expr) => self.expression(expr.inner()),
        }
    }

    /// convert expression1
    fn expression1(
        &mut self,
        expression: &cst::expression::Expression1,
    ) -> Result<ast::Expression> {
        use cst::expression::Expression1;
        match expression {
            Expression1::TypedExpression { expr, ty, .. } => {
                let (expr, ty) = {
                    let expr_res = self.expression0(expr);
                    let ty_res = self.ty(ty);
                    expr_res.combine(ty_res)?
                };
                expr.restrict_ty(ty)
                    .map_err(|e| Error::from(*e))
                    .map_err(to_errs)
            }
            Expression1::Expression0(expr) => self.expression0(expr),
        }
    }

    /// convert expression1
    pub fn expression(
        &mut self,
        expression: &cst::expression::Expression,
    ) -> Result<ast::Expression> {
        self.expression1(expression)
            .map(|expr_ast| expr_ast.with_loc(expression))
    }

    /// convert type
    pub fn ty(&mut self, ty: &cst::Type) -> Result<ast::Type> {
        match ty {
            cst::Type::Builtin(Meta { value: builtin, .. }) => {
                use cst::ty::BuiltinT;
                let builtin = match builtin {
                    BuiltinT::Nat => ast::TypeBuiltin::nat(),
                    BuiltinT::Bool => ast::TypeBuiltin::bool(),
                    BuiltinT::Char => ast::TypeBuiltin::char(),
                };
                Ok(ast::Type::Builtin(builtin))
            }
            cst::Type::Variable(ty_var) => {
                let loc = ty_var.loc();
                let id = self.id_builder.get(ty_var.name());
                let alias = self
                    .env
                    .get_alias_ty(&id, loc.clone())
                    .map_err(Error::from)
                    .map_err(to_errs)?;
                Ok(ast::Type::Alias(alias))
            }
        }
        .map(|ast_ty| ast_ty.with_loc(ty))
    }

    fn pattern(&mut self, pattern: &cst::Pattern, ty: &ast::Type) -> Result<ast::Pattern> {
        use cst::Pattern;
        match pattern {
            Pattern::Variable(var) => {
                let id = self.id_builder.build(var.name());
                self.env.add_expr_def(id.clone(), ty.clone(), var.loc());
                let pattern_var = ast::PatternVar::from(id).with_loc(var);
                Ok(ast::Pattern::Variable(pattern_var))
            }
        }
    }

    /// type expression definition
    fn expression_definition(
        &mut self,
        definition: &cst::ExpressionDefinition,
    ) -> Result<ast::ExpressionDefinition> {
        let body_res = self.expression(&definition.body);
        let ty_opt_res = definition.typed_by().map(|ty| self.ty(ty)).transpose();
        let pattern_res = {
            match (&ty_opt_res, &body_res) {
                (Ok(Some(ty)), _) => self.pattern(&definition.pattern, ty),
                (Ok(None), Ok(body)) => self.pattern(&definition.pattern, body.ty()),
                (_, _) => Err(Vec::new()),
            }
        };

        let body_res = body_res
            .combine(ty_opt_res)
            .and_then(|(body, opt_ty)| match opt_ty {
                Some(ty) => body
                    .restrict_ty(ty)
                    .map_err(|e| Error::from(*e))
                    .map_err(to_errs),
                None => Ok(body),
            });

        let (body, pattern) = body_res.combine(pattern_res)?;
        Ok(ast::ExpressionDefinition::new(pattern, body))
    }

    /// convert definition
    pub fn definition(
        &mut self,
        definition: &cst::ExpressionDefinition,
        doc_opt: Option<ast::Documentation>,
    ) -> Result<ast::ExpressionDefinition> {
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
        doc: Option<ast::Documentation>,
    ) -> Result<()> {
        let id = self.id_builder.build(definition.name.name());
        let ty = self.ty(&definition.ty)?;
        self.env
            .add_type_def(id.clone(), ty.clone(), definition.loc());
        if let Some(doc) = doc {
            self.env.set_doc(&id, doc.clone());
        }
        Ok(())
    }

    /// convert help variable
    pub fn help(&mut self, var: &cst::help::Variable) -> Result<ast::Help> {
        let id = self.id_builder.get(var.name());
        self.env
            .get_help(&id, var.loc())
            .map_err(Error::from)
            .map_err(to_errs)
    }
}
