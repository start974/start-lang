use errors::Error;

use crate::cst::{Cst, CstKind};
use crate::error::ParseError;
use crate::peg::{Class, Literal, Peg};
use crate::pratt::Pratt;
use crate::{Mark as _, Scanner};
use std::collections::HashMap;

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

type ResultParse = Result<Cst, ParseError>;

/// make a parse error with expected syntax at the given position
/// restore the scanner to the start position if parsing fails,
/// and return an error with expected syntax
fn error_expected<S>(scanner: &mut S, start: S::Mark, expected: Peg) -> ParseError
where
    S: Scanner,
{
    scanner.rollback(start);
    ParseError::new(start.position(), expected)
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
    pub fn add_pratt(mut self, name: String, rule: Pratt) -> Self {
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
        self
    }

    /// parse a literal, e.g. `'a'`
    /// restore the scanner to the start position if parsing fails, and return an error with expected syntax
    /// return a CST token if parsing succeeds
    fn parse_literal(&self, scanner: &mut impl Scanner, literal: &Literal) -> ResultParse {
        let start = scanner.checkpoint();
        let value = &literal.value;

        value
            .chars()
            .try_for_each(|expected| {
                scanner
                    .consume_if(|c| c == expected)
                    .map(|_| ())
                    .ok_or_else(|| error_expected(scanner, start, Peg::Literal(literal.clone())))
            })
            .map(|_| {
                Cst::from(CstKind::Token {
                    content: value.clone(),
                    span: scanner.span_from(start),
                })
            })
    }

    /// parse class, e.g. `[a-z]`
    /// restore the scanner to the start position if parsing fails, and return an error with expected syntax
    /// return a CST token if parsing succeeds
    fn parse_class(&self, scanner: &mut impl Scanner, class: &Class) -> ResultParse {
        scanner
            .consume_if(|c| class.is_match(c))
            .map(|c| {
                Cst::from(CstKind::Token {
                    content: c.to_string(),
                    span: scanner.span_from(scanner.checkpoint()),
                })
            })
            .ok_or_else(|| error_expected(scanner, scanner.checkpoint(), Peg::Class(class.clone())))
    }

    /// parse a PEG rule, e.g. `expr = 'a' / 'b'`
    fn parse_peg(&self, scanner: &mut impl Scanner, peg: &Peg) -> ResultParse {
        match peg {
            Peg::Literal(lit) => self.parse_literal(scanner, lit),
            Peg::Class(class) => self.parse_class(scanner, class),
            _ => unimplemented!("Only literal PEG rules are implemented"),
        }
    }

    /// parse a rule by name, e.g. `expr`
    /// fail parsing fails according to the rule's PEG or Pratt definitions
    /// can pannic if the rule is not found,
    pub fn parse(&self, scanner: &mut impl Scanner, rule_name: &str) -> Result<Cst, Error> {
        let rules = self.rules.get(rule_name).unwrap();
        let _ = self.parse_peg(scanner, &rules.atom[0]);
        todo!("Implement parsing logic for PEG and Pratt rules, and error handling")
    }
}

// ============================================================================
// Test
// ============================================================================

#[cfg(test)]
mod tests {
    use pp::{Pretty, Theme};

    use super::*;
    use crate::scanner::StringScanner;

    #[test]
    fn parse_literal() {
        let parser = Parser::default();
        let mut scanner = StringScanner::from("abcdef");
        let literal = Literal::from("abc");

        {
            let result = parser.parse_literal(&mut scanner, &literal);
            assert!(result.is_ok());

            let cst = result.unwrap();
            let theme = Theme::default();
            let cst_str = cst.make_string(&theme);
            assert_eq!(cst_str, "abc");
        }

        {
            let result = parser.parse_literal(&mut scanner, &literal);
            let err = result.err().unwrap();

            let err_expected = ParseError::new(3, Peg::Literal(literal));
            assert_eq!(err, err_expected);
        }
    }

    #[test]
    fn parse_class() {
        let parser = Parser::default();
        let mut scanner = StringScanner::from("a1");
        let class = Class::from('a'..='z');

        {
            let result = parser.parse_class(&mut scanner, &class);
            assert!(result.is_ok());

            let cst = result.unwrap();
            let theme = Theme::default();
            let cst_str = cst.make_string(&theme);
            assert_eq!(cst_str, "a");
        }

        {
            let result = parser.parse_class(&mut scanner, &class);
            let err = result.err().unwrap();

            let err_expected = ParseError::new(1, Peg::Class(class));
            assert_eq!(err, err_expected);
        }
    }
}
