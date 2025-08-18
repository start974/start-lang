use super::ast::{self, Typed as _};
use super::error::Error;
use crate::parser::cst::{self, AsCharacter as _, AsIdentifier as _, AsNumber as _};
use crate::utils::error::ResultExt as _;
use crate::utils::location::{Located as _, LocatedSet};

#[derive(Debug, Default)]
pub struct Typer {
    var_env: ast::VariableEnv,
    ty_env: ast::TypeAliasEnv,
    id_builder: ast::IdentifierBuilder,
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
        }
        .with_loc(constant)
    }

    /// convert expression0
    fn expression0(&self, expression: &cst::expression::Expression0) -> Result<ast::Expression> {
        use cst::expression::Expression0;
        match expression {
            Expression0::Constant(c) => {
                let c_ty = self.constant(c);
                Ok(ast::Expression::Constant(c_ty))
            }
            Expression0::Variable(var) => {
                let var_name = var.name();
                let id = self.id_builder.get(var_name);
                self.var_env
                    .get(&id, var.loc())
                    .map_err(Error::from)
                    .map_err(to_errs)
            }
            Expression0::Paren(expr) => self.expression(expr.inner()),
        }
    }

    /// convert expression1
    fn expression1(&self, expression: &cst::expression::Expression1) -> Result<ast::Expression> {
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
    pub fn expression(&self, expression: &cst::expression::Expression) -> Result<ast::Expression> {
        self.expression1(expression)
            .map(|expr_ast| expr_ast.with_loc(expression))
    }

    /// convert type
    pub fn ty(&self, ty: &cst::Type) -> Result<ast::Type> {
        match ty {
            cst::Type::Variable(ty_var) => {
                let name = ty_var.name();
                let id = self.id_builder.get(name);
                self.ty_env
                    .get(&id, ty_var.loc())
                    .map_err(Error::from)
                    .map_err(to_errs)
            }
        }
        .map(|ast_ty| ast_ty.with_loc(ty))
    }

    fn pattern(&mut self, pattern: &cst::Pattern, ty: &ast::Type) -> Result<ast::Pattern> {
        use cst::Pattern;
        match pattern {
            Pattern::Variable(var) => {
                let id = self.id_builder.build(var.name());
                self.var_env.add(id.clone(), ty.clone());
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
        doc: Option<ast::Documentation>,
    ) -> Result<ast::ExpressionDefinition> {
        let expr_def = self.expression_definition(definition)?;
        if let Some(doc) = doc {
            for name in expr_def.pattern().names() {
                self.var_env.add_doc(name.clone(), doc.clone());
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
        let name_parse = &definition.name;
        let id = self.id_builder.build(name_parse.name());
        let ty = self.ty(&definition.ty)?;
        if let Some(doc) = doc {
            self.ty_env.add_doc(id.clone(), doc);
        }
        self.ty_env.add(id, ty.clone());
        Ok(())
    }

    /// convert help variable
    pub fn help(&self, var: &cst::help::Variable) -> Result<ast::Help> {
        let name = var.name();
        let id = self.id_builder.get(name);
        let res_var = self
            .var_env
            .get(&id, var.loc())
            .map(|e| ast::Help {
                var: id.clone(),
                loc: var.loc(),
                info: ast::HelpInfo::Expression(e.ty().clone()),
                doc: self.var_env.get_doc(&id).cloned(),
            })
            .map_err(Error::from)
            .map_err(to_errs);

        let res_ty = self
            .ty_env
            .get(&id, var.loc())
            .map(|ty| ast::Help {
                var: id.clone(),
                loc: var.loc(),
                info: ast::HelpInfo::Type(ty.clone()),
                doc: self.ty_env.get_doc(&id).cloned(),
            })
            .map_err(Error::from)
            .map_err(to_errs);

        res_var.or(res_ty)
    }
}
