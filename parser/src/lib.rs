pub mod cst;
pub mod error;
pub mod peg;
pub mod pratt;

use std::collections::HashMap;

use error::ErrorRule;
use peg::Peg;
use pratt::Pratt;

use crate::error::ErrorRuleKind;

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
#[derive(Debug, Clone, Default)]
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
#[derive(Debug, Clone)]
struct Rules {
    /// operators in precedence order (highest precedence first)
    operators: Vec<Pratt>,
    // base expression (literal, ident, parens, etc.)
    atom: Vec<Peg>,
}

impl Parser {
    /// add peg rule to the parser, e.g. `expr = 'a' / 'b' / '(' expr ')'`
    pub fn add_peg(mut self, name: String, rule: Peg) -> Result<Self, ErrorRule> {
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

    fn rule_exist(&self, name: &str) -> bool {
        self.rules.contains_key(name)
    }

    /*    fn check_rule_peg(&self, rule: &Peg) -> Result<(), Vec<ErrorRule>> {*/
    /*match rule.kind() {*/
    /*peg::Kind::Literal(_) | peg::Kind::Class(_) => Ok(()),*/
    /*peg::Kind::RuleRef(name) => {*/
    /*if self.rule_exist(name) {*/
    /*Ok(())*/
    /*} else {*/
    /*Err(vec![ErrorRule::from(ErrorRuleKind::RuleNotExist(*/
    /*name.clone(),*/
    /*))])*/
    /*}*/
    /*}*/
    /*peg::Kind::Seq(pegs) => pegs*/
    /*.iter()*/
    /*.map(|peg| self.check_rule_peg(peg))*/
    /*.collect::<Result<(), Vec<ErrorRule>>>(),*/
    /*peg::Kind::Choice(pegs) => pegs*/
    /*.iter()*/
    /*.map(|peg| self.check_rule_peg(peg))*/
    /*.collect::<Result<(), Vec<ErrorRule>>>(),*/
    /*peg::Kind::Group(peg) => {*/
    /*self.check_rule_peg(peg)*/
    /*}*/
    /*peg::Kind::Optional(peg) => {*/
    /*self.check_rule_peg(peg)*/
    /*}*/
    /*peg::Kind::Repeat0(peg) => {*/

    /*self.check_rule_peg(peg)*/
    /*}*/
    /*peg::Kind::Repeat1(peg) => {*/
    /*self.check_rule_peg(peg)*/
    /*}*/
    /*peg::Kind::RepeatRange(peg, _, _) => todo!(),*/
    /*peg::Kind::NegativeLookahead(peg) => todo!(),*/
    /*peg::Kind::PositiveLookahead(peg) => todo!(),*/
    /*   }*/
    //}

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
