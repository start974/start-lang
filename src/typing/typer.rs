use super::ast::{self as ast_typed, IdentifierBuilder, Type, TypeAliasEnv, Typed, VariableEnv};
use super::error::Error;
use crate::parser::ast as ast_parser;
use crate::utils::location::LocatedSet;

#[derive(Debug, Default)]
pub struct Typer {
    var_env: VariableEnv,
    ty_alias: TypeAliasEnv,
    id_builder: IdentifierBuilder,
}

type Result<T, E = Box<Error>> = std::result::Result<T, E>;

impl Typer {
    /// convert constant
    pub fn constant(&self, constant: &ast_parser::Constant) -> ast_typed::Constant {
        match constant.kind() {
            ast_parser::ConstantKind::Nat(n) => ast_typed::Constant::nat(n.clone()),
            ast_parser::ConstantKind::Char(c) => ast_typed::Constant::character(*c),
        }
        .with_loc(constant)
    }

    /// convert expression
    pub fn expression(&self, expression: &ast_parser::Expression) -> Result<ast_typed::Expression> {
        match &expression.kind {
            ast_parser::ExpressionKind::Constant(c) => {
                let c_ty = self.constant(c);
                Ok(ast_typed::Expression::Constant(c_ty))
            }
            ast_parser::ExpressionKind::Variable(x) => {
                let id = self.id_builder.get(x.name()).with_loc(x);
                self.var_env
                    .get(&id)
                    .map_err(Error::from)
                    .map(ast_typed::Expression::from)
                    .or_else(|e| match x.name() {
                        "__Constant_true__" => {
                            let b = ast_typed::Constant::boolean(true);
                            Ok(ast_typed::Expression::from(b))
                        }
                        "__Constant_false__" => {
                            let b = ast_typed::Constant::boolean(false);
                            Ok(ast_typed::Expression::from(b))
                        }
                        _ => Err(e),
                    })
            }
            ast_parser::ExpressionKind::TypeRestriction(ty_restr) => {
                // TODO: make multiple error
                let expr = self.expression(ty_restr.expression())?;
                let ty = self.ty(ty_restr.ty())?;
                expr.restrict_ty(ty).map_err(|e| Error::from(*e))
            }
        }
        .map_err(Box::new)
    }

    /// convert typ
    pub fn ty(&self, ty: &ast_parser::Type) -> Result<ast_typed::Type> {
        match ty {
            ast_parser::Type::Var(ident) => {
                let name = ident.name();
                let id = self.id_builder.get(name).with_loc(ident);
                self.ty_alias
                    .get(&id)
                    .map(Type::from)
                    .or_else(|e| match ident.name() {
                        "__Type_Nat__" => Ok(ast_typed::Type::from(ast_typed::TypeBuiltin::Nat)),
                        "__Type_Bool__" => Ok(ast_typed::Type::from(ast_typed::TypeBuiltin::Bool)),
                        "__Type_Char__" => Ok(ast_typed::Type::from(ast_typed::TypeBuiltin::Char)),
                        _ => Err(e),
                    })
                    .map_err(Error::from)
                    .map_err(Box::new)
            }
        }
    }

    /// convert definition
    pub fn expression_definition(
        &mut self,
        definition: &ast_parser::ExpressionDefinition,
    ) -> Result<ast_typed::ExpressionDefinition> {
        let name_parse = definition.name();
        let name = self
            .id_builder
            .build(name_parse.name())
            .with_loc(name_parse);
        match definition.ty() {
            Some(ty) => {
                let ty = self.ty(ty)?;
                self.var_env.add(name.clone(), ty.clone());
                let body = self.expression(definition.body())?;
                ast_typed::ExpressionDefinition::new(name, body)
                    .restrict_ty(ty)
                    .map_err(|e| Error::from(*e))
                    .map_err(Box::new)
            }
            None => {
                let body = self.expression(definition.body())?;
                self.var_env.add(name.clone(), body.ty().clone());
                Ok(ast_typed::ExpressionDefinition::new(name, body))
            }
        }
    }

    /// add type definition
    pub fn type_definition(&mut self, definition: &ast_parser::TypeDefinition) -> Result<()> {
        let name_parse = definition.name();
        let name = self
            .id_builder
            .build(name_parse.name())
            .with_loc(name_parse);
        let ty = self.ty(definition.ty())?;
        self.ty_alias.add(name.clone(), ty.clone());
        Ok(())
    }
}
