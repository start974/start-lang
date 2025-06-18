use super::ast::{self as ast_typed, IdentifierBuilder, Type, TypeAliasEnv, Typed, VariableEnv};
use super::error::ErrorFromParser;
use crate::parser::ast as ast_parser;
use crate::utils::location::LocatedSet;

#[derive(Debug)]
pub struct Typer {
    var_env: VariableEnv,
    ty_alias: TypeAliasEnv,
    id_builder: IdentifierBuilder,
}

type Error = ErrorFromParser;
type Result<T, E = Error> = std::result::Result<T, E>;

impl Typer {
    /// make a new structure to from parser
    pub fn new() -> Self {
        Self {
            var_env: VariableEnv::new(),
            ty_alias: TypeAliasEnv::new(),
            id_builder: IdentifierBuilder::new(),
        }
    }

    /// convert constant
    pub fn constant(&self, constant: &ast_parser::Constant) -> ast_typed::Constant {
        match constant.kind() {
            ast_parser::ConstantKind::N(n) => ast_typed::Constant::n(n.clone()),
        }
        .with_loc(constant)
    }

    /// convert expression
    pub fn expression(&self, expression: &ast_parser::Expression) -> Result<ast_typed::Expression> {
        match expression {
            ast_parser::Expression::Constant(c) => {
                let c_ty = self.constant(c);
                Ok(ast_typed::Expression::Constant(c_ty))
            }
            ast_parser::Expression::Variable(x) => {
                let id = self.id_builder.get(x.name()).with_loc(x);
                self.var_env
                    .get(&id)
                    .map_err(Error::from)
                    .map(ast_typed::Expression::from)
                    .or_else(|e| match x.name() {
                        "__Constant_true__" => {
                            Ok(ast_typed::Expression::from(ast_typed::Constant::b(true)))
                        }
                        "__Constant_false__" => {
                            Ok(ast_typed::Expression::from(ast_typed::Constant::b(false)))
                        }
                        _ => Err(e),
                    })
            }
            ast_parser::Expression::TypeRestriction(ty_restr) => {
                // TODO: make multiple error
                let expr = self.expression(ty_restr.expression())?;
                let ty = self.ty(ty_restr.ty())?;
                expr.restrict_ty(ty).map_err(Error::from)
            }
        }
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
                        "__Type_N__" => Ok(ast_typed::Type::from(ast_typed::TypeBuiltin::N)),
                        "__Type_B__" => Ok(ast_typed::Type::from(ast_typed::TypeBuiltin::B)),
                        _ => Err(e),
                    })
                    .map_err(Error::from)
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
                    .map_err(Error::from)
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

impl Default for Typer {
    fn default() -> Self {
        Self::new()
    }
}
