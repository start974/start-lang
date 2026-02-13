use std::collections::HashMap;
use crate::Scanner;
use crate::peg::Peg;
use crate::pratt::Pratt;

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
    pub fn add_peg(mut self, name: String, rule: Peg) -> Self {
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
        self
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
