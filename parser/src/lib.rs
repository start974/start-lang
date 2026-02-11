pub mod cst;
pub mod error;
pub mod peg;
pub mod pratt;

use std::collections::HashMap;

use errors::{Errors, ResultErrors as _};
use location::GetSpan as _;
use peg::Peg;
use pratt::Pratt;

pub trait Scanner {
    /// peek the next character without consuming it
    fn peek(&self) -> Option<char>;

    /// consume and return the next character
    fn next(&mut self) -> Option<char>;

    /// make a checkpoint of the current position in the input (for backtracking)
    fn checkpoint(&self) -> usize;

    /// restore the input to a previous position (backtracking)
    fn rollback(&mut self, pos: usize);
}

/// Full parser containing all grammar rules.
///
/// The parser supports:
/// - PEG rules (standard grammar)
/// - Pratt rules (precedence-based parsing)
///
/// Both are unified under the same rule table to avoid splitting the logic.
#[derive(Debug, Default)]
pub struct Parser {
    /// All grammar rules indexed by name.
    ///
    /// Each rule may be:
    /// - a PEG syntax rule
    /// - or a Pratt expression rule
    rules: HashMap<String, Rules>,
}

/// A grammar rule.
/// Peg and Pratt rules
#[derive(Debug)]
struct Rules {
    /// operators in precedence order (highest precedence first)
    operators: Vec<Pratt>,
    // base expression (literal, ident, parens, etc.)
    atom: Vec<Peg>,
}

impl Parser {
    /// add peg rule to the parser, e.g. `expr = 'a' / 'b' / '(' expr ')'`
    pub fn add_peg(mut self, name: String, rule: Peg) -> Result<Self, Errors> {
        self.check_peg(&rule)?;
        if let Some(rules) = self.rules.get_mut(&name) {
            rules.atom.push(rule);
        } else {
            self.rules.insert(
                name,
                Rules {
                    operators: Vec::new(),
                    atom: vec![rule],
                },
            );
        }
        Ok(self)
    }

    /// check if a peg rule is defined
    fn rule_exist(&self, name: &str) -> bool {
        self.rules.contains_key(name)
    }

    /*    /// check if a peg rule can match an empty string, e.g. `expr = ''` or `expr = expr?`*/
    /*/// fail if rule is not defined, e.g. `expr = other` where `other` is not defined*/
    /*fn nullable(&self, rule: &Peg) -> bool {*/
    /*use peg::Kind::*;*/
    /*match rule.kind() {*/
    /*Literal(s) => s.is_empty(),*/
    /*Class(_) => false,*/
    /*RuleRef(name) => self*/
    /*.rules*/
    /*.get(name)*/
    /*.unwrap_or(false)*/
    /*.atom*/
    /*.iter()*/
    /*.any(|r| self.nullable(r)),*/
    /*Seq(pegs) => pegs.iter().all(|r| self.nullable(r)),*/
    /*Choice(pegs) => pegs.iter().any(|r| self.nullable(r)),*/
    /*Repeat(rep) => rep.min() == 0,*/
    /*Group(peg) => self.nullable(peg),*/
    /*NegativeLookahead(_) | PositiveLookahead(_) => true,*/
    /*}*/
    /*}*/

    fn check_peg(&self, rule: &Peg) -> Result<(), Errors> {
        match rule.kind() {
            peg::Kind::Literal(_) | peg::Kind::Class(_) => Ok(()),
            peg::Kind::RuleRef(name) => {
                if self.rule_exist(name) {
                    Ok(())
                } else {
                    Err(error::not_defined(name, rule.span()).into())
                }
            }
            peg::Kind::Seq(pegs) | peg::Kind::Choice(pegs) =>
            {
                #[allow(clippy::manual_try_fold)]
                pegs.iter()
                    .fold(Ok(()), |acc, x| acc.combine(self.check_peg(x)).map(|_| ()))
            }

            peg::Kind::Repeat(rep) => self.check_peg(rep.rule()),
            peg::Kind::Group(peg)
            | peg::Kind::NegativeLookahead(peg)
            | peg::Kind::PositiveLookahead(peg) => self.check_peg(peg),
        }
    }

    /// add pratt rule to the parser, e.g. `expr = expr '+' expr`
    /// with precedence 10 and left associativity
    pub fn add_pratt(&mut self, name: String, rule: Pratt) {
        if let Some(rules) = self.rules.get_mut(&name) {
            rules.operators.push(rule);
        } else {
            self.rules.insert(
                name,
                Rules {
                    operators: vec![rule],
                    atom: Vec::new(),
                },
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use peg::Kind::*;

    #[test]
    fn test_add_peg() {
        let parser = Parser::default()
            .add_peg("expr".to_string(), Literal("a".to_string()).into())
            .unwrap()
            .add_peg("expr".to_string(), Literal("b".to_string()).into())
            .unwrap();

        assert!(parser.rule_exist("expr"));
        assert_eq!(parser.rules["expr"].atom.len(), 2);
    }

    #[test]
    fn test_add_peg_not_exist() {
        let parser =
            Parser::default().add_peg("expr".to_string(), RuleRef("other".to_string()).into());
        assert!(parser.is_err());
    }
}
