use core::fmt;

use super::rule::Rule;
use crate::frontend::expression::Expr;

#[derive(Clone)]
pub struct NotationRule {
    name: String,
    rule: Rule,
    exp: Expr,
}

impl NotationRule {
    pub fn name(&self) -> String{
        self.name
    }

    pub fn rule(&self) -> Rule {
        self.rule
    }   

    pub fn expression(&self) -> Expr {
        self.exp
    }   
}

impl fmt::Display for NotationRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Notation Rule {}: {} :=\n\t{}",
            self.name, self.rule, "exp" /*self.exp*/
        )
    }
}
