use super::ast::{self, Typed as _};
use super::error::Error;
use crate::parser::cst::{self, AsCharacter as _, AsIdentifier as _, AsNumber as _};
use crate::utils::location::LocatedSet;

#[derive(Debug, Default)]
pub struct Typer {
    var_env: ast::VariableEnv,
    ty_alias: ast::TypeAliasEnv,
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
                self.var_env
                    .get(&id)
                    .map_err(Error::from)
                    .map(ast::Expression::from)
                    .or_else(|e| match var_name {
                        "__Constant_true__" => {
                            let b = ast::Constant::boolean(true);
                            Ok(ast::Expression::from(b))
                        }
                        "__Constant_false__" => {
                            let b = ast::Constant::boolean(false);
                            Ok(ast::Expression::from(b))
                        }
                        _ => Err(e),
                    })
                    .map_err(Box::new)
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
    }

    /// convert type
    pub fn ty(&self, ty: &cst::Type) -> Result<ast::Type> {
        match ty {
            cst::Type::Variable(ident) => {
                let name = ident.name();
                let id = self.id_builder.get(name).with_loc(ident);
                self.ty_alias
                    .get(&id)
                    .map(ast::Type::from)
                    .or_else(|e| {
                        match name {
                            "__Type_Nat__" => Ok(ast::TypeBuiltin::nat()),
                            "__Type_Bool__" => Ok(ast::TypeBuiltin::bool()),
                            "__Type_Char__" => Ok(ast::TypeBuiltin::char()),
                            _ => Err(e),
                        }
                        .map(ast::Type::from)
                    })
                    .map_err(Error::from)
                    .map_err(Box::new)
            }
        }
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
        _doc: Option<ast::Documentation>,
    ) -> Result<ast::ExpressionDefinition> {
        let expr_def = self.expression_definition(definition)?;
        Ok(expr_def)
    }

    /// add type definition
    pub fn type_definition(
        &mut self,
        definition: &cst::TypeDefinition,
        _doc: Option<ast::Documentation>,
    ) -> Result<()> {
        let name_parse = &definition.name;
        let name = self
            .id_builder
            .build(name_parse.name())
            .with_loc(name_parse);
        let ty = self.ty(&definition.ty)?;
        self.ty_alias.add(name.clone(), ty.clone());
        Ok(())
    }
}
