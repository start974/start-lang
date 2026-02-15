use errors::Error;

use crate::cst::{Cst, CstKind};
use crate::error::ParseError;
use crate::peg::{Class, Literal, Peg};
use crate::pratt::Pratt;
use crate::{Mark as _, Scanner};
use std::collections::{BTreeMap, HashMap};

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
    rules: HashMap<String, RuleGroup>,
}

/// A grammar rule.
/// Peg and Pratt rules
#[derive(Debug)]
struct RuleGroup {
    /// operators grouped by precedence
    /// key = precedence level
    /// higher key = stronger binding
    operators: BTreeMap<usize, Vec<Pratt>>,

    /// base expressions (literal, ident, parens, etc.)
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
    pub fn add_peg(mut self, name: &str, rule: Peg) -> Self {
        match self.rules.get_mut(name) {
            Some(rules) => {
                rules.atom.push(rule);
            }
            None => {
                self.rules.insert(
                    name.into(),
                    RuleGroup {
                        operators: BTreeMap::new(),
                        atom: vec![rule],
                    },
                );
            }
        }
        self
    }

    /// add pratt rule to the parser, e.g. `expr = expr '+' expr`
    pub fn add_pratt(mut self, name: &str, rule: Pratt) -> Self {
        let group = self.rules.entry(name.into()).or_insert_with(|| RuleGroup {
            operators: BTreeMap::new(),
            atom: Vec::new(),
        });

        group
            .operators
            .entry(rule.precedence)
            .or_default()
            .push(rule);

        self
    }

    /// parse a literal, e.g. `'a'`
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

    /*    /// parse reference to another rule, e.g. `expr`*/
    /*fn parse_ref_rule(&self, scanner: &mut impl Scanner, ref_rule: &RefRule) -> ResultParse {*/
    /*let name = &ref_rule.name;*/
    /*let rules = self*/
    /*.rules*/
    /*.get(name)*/
    /*.unwrap_or_else(|| panic!("Rule \"{name}\" not found in parser rules"));*/
    /*let start = scanner.checkpoint();*/
    /*self.parse_rules(scanner, rules).map_err(|err| {*/
    /*if err.position() == start.position() {*/
    /*// If the error is at the same position, it means the rule was found but parsing failed*/
    /*error_expected(scanner, start, Peg::RefRule(ref_rule.clone()))*/
    /*} else {*/
    /*// If the error is at a different position, it means the rule was found and parsing was attempted, so we keep the original error*/
    /*err*/
    /*}*/
    /*})*/
    /*}*/

    /// parse a PEG rule, e.g. `expr = 'a' / 'b'`
    fn parse_peg(&self, scanner: &mut impl Scanner, peg: &Peg) -> ResultParse {
        match peg {
            Peg::Literal(lit) => self.parse_literal(scanner, lit),
            Peg::Class(class) => self.parse_class(scanner, class),
            //Peg::RefRule(rule_name) => self.parse_ref_rule(scanner, rule_name),
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

/*    #[test]*/
    /*fn parse_ref_rule() {*/
        /*let parser = Parser::default().add_peg("expr", Peg::Literal(Literal::from("abc")));*/
        /*let mut scanner = StringScanner::from("abcdef");*/

        /*{*/
            /*let result = parser.parse_ref_rule(&mut scanner, &RefRule::from("expr"));*/
            /*assert!(result.is_ok());*/

            /*let cst = result.unwrap();*/
            /*let theme = Theme::default();*/
            /*let cst_str = cst.make_string(&theme);*/
            /*assert_eq!(cst_str, "abc");*/
        /*}*/

        /*{*/
            /*let result = parser.parse_ref_rule(&mut scanner, &RefRule::from("expr"));*/
            /*let err = result.err().unwrap();*/

            /*let err_expected = ParseError::new(3, Peg::Literal(Literal::from("abc")));*/
            /*assert_eq!(err, err_expected);*/
        /*}*/
    /*}*/
}
