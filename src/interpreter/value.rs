pub use crate::ast::pretty_print::*;
use crate::ast::{Ident, NConst, Ty};
use crate::error::*;
use crate::location::*;

#[derive(Debug, Clone)]
pub enum Value {
    N(NConst),
}

const ERROR_CONVERT_TO_INTEGER: i32 = -2;
impl TryInto<i32> for Value {
    type Error = Error;
    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Self::N(n) => NConst::try_into(n).map_err(|_| {
                let msg = Head::new().text("value is to loong to be convert to integer");
                Error::make(msg, ERROR_CONVERT_TO_INTEGER)
            }),
        }
    }
}

impl Pretty for Value {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Self::N(n) => theme.number(n),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DefValue {
    pub name: Ident,
    pub ty: Ty,
    pub value: Value,
    pub location: Option<Location>,
}

impl Pretty for DefValue {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::group(
            Doc::nil()
                .append(theme.def_var(&self.name))
                .append(Doc::space())
                .append(Doc::group(
                    theme
                        .op_typed_by()
                        .append(Doc::line())
                        .append(self.ty.pretty(theme)),
                ))
                .append(Doc::space())
                .append(theme.op_eq_def())
                .append(Doc::line())
                .append(self.value.pretty(theme)),
        )
    }
}

impl Located for DefValue {
    fn get_location(&self) -> &Option<Location> {
        &self.location
    }

    fn set_opt_location(mut self, opt_location: Option<Location>) -> Self {
        self.location = opt_location;
        self
    }
}

pub type DefValues = Vec<DefValue>;

pub enum DefsOrValue {
    Value(Value),
    Defs(DefValues),
}

impl Pretty for DefsOrValue {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Self::Value(value) => value.pretty(theme),
            Self::Defs(defs) => defs.iter().fold(Doc::nil(), |doc, def| {
                doc.append(def.pretty(theme).append(Doc::line_()))
            }),
        }
    }
}
