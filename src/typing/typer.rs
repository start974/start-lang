use super::ast::{self, Typed as _};
use super::error::Error;
use crate::parser::cst;
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
    pub fn constant(&self, constant: &cst::Constant) -> ast::Constant {
        match constant.kind() {
            cst::ConstantKind::Nat(n) => ast::Constant::nat(n.clone()),
            cst::ConstantKind::Char(c) => ast::Constant::character(*c),
        }
        .with_loc(constant)
    }

    /// convert expression
    pub fn expression(&self, expression: &cst::Expression) -> Result<ast::Expression> {
        match expression {
            cst::Expression::Constant(c) => {
                let c_ty = self.constant(c);
                Ok(ast::Expression::Constant(c_ty))
            }
            cst::Expression::Variable(var) => {
                let var_name = var.to_string();
                let id = self.id_builder.get(&var_name).with_loc(var);
                self.var_env
                    .get(&id)
                    .map_err(Error::from)
                    .map(ast::Expression::from)
                    .or_else(|e| match var_name.as_str() {
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
            }
            cst::Expression::TypeRestriction(ty_restr) => {
                // TODO: make multiple error
                let expr = self.expression(ty_restr.expression())?;
                let ty = self.ty(ty_restr.ty())?;
                expr.restrict_ty(ty).map_err(|e| Error::from(*e))
            }
        }
        .map_err(Box::new)
    }

    /// convert typ
    pub fn ty(&self, ty: &cst::Type) -> Result<ast::Type> {
        match ty {
            cst::Type::Var(ident) => {
                let name = ident.to_string();
                let id = self.id_builder.get(&name).with_loc(ident);
                self.ty_alias
                    .get(&id)
                    .map(ast::Type::from)
                    .or_else(|e| {
                        match name.as_str() {
                            "__Type_Nat__" => Ok(ast::TypeBuiltin::Nat),
                            "__Type_Bool__" => Ok(ast::TypeBuiltin::Bool),
                            "__Type_Char__" => Ok(ast::TypeBuiltin::Char),
                            _ => Err(e),
                        }
                        .map(ast::Type::from)
                    })
                    .map_err(Error::from)
                    .map_err(Box::new)
            }
        }
    }

    /// convert definition
    pub fn expression_definition(
        &mut self,
        definition: &cst::ExpressionDefinition,
    ) -> Result<ast::ExpressionDefinition> {
        match definition.pattern() {
            cst::Pattern::Variable(var) => {
                let name_parse = var.to_string();
                let name = self.id_builder.build(&name_parse).with_loc(var);
                match definition.ty() {
                    Some(ty) => {
                        let ty = self.ty(ty)?;
                        self.var_env.add(name.clone(), ty.clone());
                        let body = self.expression(definition.body())?;
                        ast::ExpressionDefinition::new(name, body)
                            .restrict_ty(ty)
                            .map_err(|e| Error::from(*e))
                            .map_err(Box::new)
                    }
                    None => {
                        let body = self.expression(definition.body())?;
                        self.var_env.add(name.clone(), body.ty().clone());
                        Ok(ast::ExpressionDefinition::new(name, body))
                    }
                }
            }
        }
    }

    /// add type definition
    pub fn type_definition(&mut self, definition: &cst::TypeDefinition) -> Result<()> {
        let name_parse = definition.name();
        let name = self
            .id_builder
            .build(&name_parse.to_string())
            .with_loc(name_parse);
        let ty = self.ty(definition.ty())?;
        self.ty_alias.add(name.clone(), ty.clone());
        Ok(())
    }
}
