use super::ast::{self, Typed as _};
use super::error::Error;
use crate::parser::cst::{self, AsCharacter as _, AsIdentifier as _, AsNumber as _};
use crate::utils::location::LocatedSet;

#[derive(Debug, Default)]
pub struct Typer {
    var_env: ast::VariableEnv,
    ty_env: ast::TypeAliasEnv,
    id_builder: ast::IdentifierBuilder,
}

type Result<T, E = Box<Error>> = std::result::Result<T, E>;

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
                let id = self.id_builder.get(var_name).with_loc(var);
                self.var_env.get(&id).map_err(Error::from).map_err(Box::new)
            }
            Expression0::Paren(expr) => self.expression(expr.inner()),
        }
    }

    /// convert expression1
    fn expression1(&self, expression: &cst::expression::Expression1) -> Result<ast::Expression> {
        use cst::expression::Expression1;
        match expression {
            Expression1::TypedExpression { expr, ty, .. } => {
                // TODO: make multiple error
                let expr = self.expression0(expr)?;
                let ty = self.ty(ty)?;
                expr.restrict_ty(ty)
                    .map_err(|e| Error::from(*e))
                    .map_err(Box::new)
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
            cst::Type::Variable(ident) => {
                let name = ident.name();
                let id = self.id_builder.get(name).with_loc(ident);
                self.ty_env.get(&id).map_err(Error::from).map_err(Box::new)
            }
        }
        .map(|ast_ty| ast_ty.with_loc(ty))
    }

    /// type expression definition
    fn expression_definition(
        &mut self,
        definition: &cst::ExpressionDefinition,
    ) -> Result<ast::ExpressionDefinition> {
        match &definition.pattern {
            cst::Pattern::Variable(var) => {
                let name_parse = var.name();
                let name = self.id_builder.build(name_parse).with_loc(var);
                match definition.typed_by() {
                    Some(ty) => {
                        let ty = self.ty(ty)?;
                        self.var_env.add(name.clone(), ty.clone());
                        let body = self.expression(&definition.body)?;
                        ast::ExpressionDefinition::new(name, body)
                            .restrict_ty(ty)
                            .map_err(|e| Error::from(*e))
                            .map_err(Box::new)
                    }
                    None => {
                        let body = self.expression(&definition.body)?;
                        self.var_env.add(name.clone(), body.ty().clone());
                        Ok(ast::ExpressionDefinition::new(name, body))
                    }
                }
            }
        }
    }

    /// convert definition
    pub fn definition(
        &mut self,
        definition: &cst::ExpressionDefinition,
        doc: Option<ast::Documentation>,
    ) -> Result<ast::ExpressionDefinition> {
        let expr_def = self.expression_definition(definition)?;
        if let Some(doc) = doc {
            self.var_env.add_doc(expr_def.name().clone(), doc);
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
        let name = self
            .id_builder
            .build(name_parse.name())
            .with_loc(name_parse);
        let ty = self.ty(&definition.ty)?;
        self.ty_env.add(name.clone(), ty.clone());
        if let Some(doc) = doc {
            self.ty_env.add_doc(name.clone(), doc);
        }
        Ok(())
    }

    /// convert help variable
    pub fn help(&self, var: &cst::help::Variable) -> Result<ast::Help> {
        let name = var.name();
        let id = self.id_builder.get(name).with_loc(var);
        let res_var = self
            .var_env
            .get(&id)
            .map(|e| ast::Help {
                var: id.clone(),
                info: ast::HelpInfo::Expression(e.ty().clone()),
                doc: self.var_env.get_doc(&id).cloned(),
            })
            .map_err(Error::from)
            .map_err(Box::new);

        let res_ty = self
            .ty_env
            .get(&id)
            .map(|ty| ast::Help {
                var: id.clone(),
                info: ast::HelpInfo::Alias(ty.clone()),
                doc: self.ty_env.get_doc(&id).cloned(),
            })
            .map_err(Error::from)
            .map_err(Box::new);

        res_var.or(res_ty)
    }
}
