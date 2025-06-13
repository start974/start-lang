use super::ast::{self as ast_typed, IdentifierBuilder, Ty, TyAliasEnv, Typed, VariableEnv};
use super::error::ErrorFromParser;
use crate::parser::ast as ast_parser;
use crate::utils::location::LocatedSet;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

pub struct FromParser {
    var_env: VariableEnv,
    ty_alias: TyAliasEnv,
    id_builder: IdentifierBuilder,
}

type Error = ErrorFromParser;
type Result<T, E = Error> = std::result::Result<T, E>;

impl FromParser {
    /// make a new structure to from parser
    pub fn new() -> Self {
        Self {
            var_env: VariableEnv::new(),
            ty_alias: TyAliasEnv::new(),
            id_builder: IdentifierBuilder::new(),
        }
    }

    /// convert constant
    fn constant(&self, constant: &ast_parser::Constant) -> ast_typed::Constant {
        match constant.kind() {
            ast_parser::ConstantKind::N(n) => ast_typed::Constant::n(n.clone()),
            //ast_parser::ConstantKind::B(b) => ast_typed::Constant::b(*b),
        }
        .with_loc(constant)
    }

    /// convert expression
    pub fn expression(&self, expression: &ast_parser::Expression) -> Result<ast_typed::Expression> {
        let expr = match expression.kind() {
            ast_parser::ExpressionKind::Constant(c) => {
                let c_ty = self.constant(c);
                ast_typed::Expression::Constant(c_ty)
            }
            ast_parser::ExpressionKind::Variable(x) => {
                let id = self.id_builder.get(x.name()).with_loc(x);
                let var = self.var_env.get(&id).map_err(Error::from)?;
                ast_typed::Expression::Variable(var)
            }
        }
        .with_loc(expression);
        match expression.ty() {
            Some(ty) => {
                let ty = self.ty(ty)?;
                expr.constraint_ty(ty).map_err(Error::from)
            }
            None => Ok(expr),
        }
    }

    /// convert typ
    pub fn ty(&self, ty: &ast_parser::Ty) -> Result<ast_typed::Ty> {
        match ty {
            ast_parser::Ty::Var(ident) => {
                let id = self.id_builder.get(ident.name()).with_loc(ident);
                self.ty_alias.get(&id).map_err(Error::from).map(Ty::from)
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
                    .constraint_ty(ty)
                    .map_err(Error::from)
            }
            None => {
                let body = self.expression(definition.body())?;
                self.var_env.add(name.clone(), body.ty().clone());
                Ok(ast_typed::ExpressionDefinition::new(name, body))
            }
        }
    }

    /// add ty definition
    pub fn ty_definition(&mut self, definition: &ast_parser::TyDefinition) -> Result<()> {
        let name_parse = definition.name();
        let name = self
            .id_builder
            .build(name_parse.name())
            .with_loc(name_parse);
        let ty = self.ty(definition.ty())?;
        self.ty_alias.add(name.clone(), ty.clone());
        Ok(())
    }

    /// convert program from parser to typed
    pub fn program(
        &mut self,
        program: &ast_parser::Program,
    ) -> Result<ast_typed::Program, Vec<Error>> {
        let mut errors = Vec::new();
        let mut program_typed = ast_typed::Program::new();
        for item in program.iter() {
            match item {
                ast_parser::ProgramItem::TyDef(def) => {
                    self.ty_definition(def).unwrap_or_else(|e| errors.push(e))
                }
                ast_parser::ProgramItem::ExprDef(def) => match self.expression_definition(def) {
                    Ok(def) => {
                        let def_name = def.name().clone();
                        program_typed = program_typed
                            .with_definition(def)
                            .unwrap_or_else(|| panic!("Definition {:#?} already add.", def_name))
                    }
                    Err(e) => errors.push(e),
                },
            }
        }
        if errors.is_empty() {
            Ok(program_typed)
        } else {
            Err(errors)
        }
    }
}

impl Default for FromParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Pretty for FromParser {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::nil()
            .append(Doc::line_())
            .append(Doc::text("Env var:"))
            .append(Doc::line_())
            .append(self.var_env.pretty(theme))
            .append(Doc::line_())
            .append(Doc::text("Env ty alias:"))
            .append(Doc::line_())
            .append(self.ty_alias.pretty(theme))
            .append(Doc::line_())
            .append(Doc::text("Identifier Builder:"))
            .append(Doc::line_())
            .append(self.id_builder.pretty(theme))
    }
}
