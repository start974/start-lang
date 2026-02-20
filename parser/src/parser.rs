use errors::Error;

use crate::cst::Cst;
use crate::error::ParseError;
use crate::peg::{Class, Literal, Peg, RefRule};
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
    atoms: Vec<Peg>,
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
                rules.atoms.push(rule);
            }
            None => {
                self.rules.insert(
                    name.into(),
                    RuleGroup {
                        operators: BTreeMap::new(),
                        atoms: vec![rule],
                    },
                );
            }
        }
        self
    }

    /// add pratt rule to the parser, e.g. `expr = expr '+' expr`
    pub fn add_pratt(mut self, name: &str, rule: Pratt) -> Self {
        assert!(
            rule.precedence > 0,
            "Pratt rules must have a precedence greater than 0"
        );
        let group = self.rules.entry(name.into()).or_insert_with(|| RuleGroup {
            operators: BTreeMap::new(),
            atoms: Vec::new(),
        });

        group
            .operators
            .entry(rule.precedence)
            .or_default()
            .push(rule);

        self
    }

    ///// parse whitespace
    /*    fn parse_ws(&self, scanner: &mut impl Scanner, ws: &WhiteSpace) -> ResultParse {*/
    /*let start = scanner.checkpoint();*/
    /*scanner*/
    /*.consume_if(|c| c.is_whitespace())*/
    /*.map(|c| {*/
    /*Cst::from(CstKind::Token {*/
    /*content: c.into(),*/
    /*span: scanner.span_from(start),*/
    /*})*/
    /*})*/
    /*.ok_or_else(|| error_expected(scanner, start, ws.clone().into()))*/
    /*}*/

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
                    .ok_or_else(|| error_expected(scanner, start, literal.clone().into()))
            })
            .map(|_| Cst::token(value, scanner.span_from(start)))
    }

    /// parse class, e.g. `[a-z]`
    fn parse_class(&self, scanner: &mut impl Scanner, class: &Class) -> ResultParse {
        scanner
            .consume_if(|c| class.is_match(c))
            .map(|c| Cst::token(&c.to_string(), scanner.span_from(scanner.checkpoint())))
            .ok_or_else(|| error_expected(scanner, scanner.checkpoint(), class.clone().into()))
    }

    /// parse reference to another rule, e.g. `expr`
    fn parse_ref_rule(&self, scanner: &mut impl Scanner, ref_rule: &RefRule) -> ResultParse {
        let name = &ref_rule.name;
        let r_group = self
            .rules
            .get(name)
            .unwrap_or_else(|| panic!("Rule \"{name}\" not found in parser rules"));
        let start = scanner.checkpoint();
        self.parse_rule_group(scanner, r_group).map_err(|err| {
            if err.position() == start.position() {
                error_expected(scanner, start, ref_rule.clone().into())
            } else {
                err
            }
        })
    }

    /// parse choice between multiple PEG rules, e.g. `= 'a' / 'b' / '(' expr ')'`
    fn parse_choice(&self, scanner: &mut impl Scanner, pegs: &[Peg]) -> ResultParse {
        self.parse_or(scanner, pegs)
            .unwrap_or_else(|| panic!("Peg rule is empty, expected at least one choice"))
    }

    /// parse a PEG rule, e.g. `expr = 'a' / 'b'`
    fn parse_peg(&self, scanner: &mut impl Scanner, peg: &Peg) -> ResultParse {
        match peg {
            Peg::Literal(lit) => self.parse_literal(scanner, lit),
            Peg::Class(class) => self.parse_class(scanner, class),
            Peg::RefRule(rule_name) => self.parse_ref_rule(scanner, rule_name),
            Peg::Choice(pegs) => self.parse_choice(scanner, pegs),
            _ => unimplemented!("Not implemented yet"),
        }
    }

    /// parse a PEG rule group, e.g. `expr = 'a' / 'b' / '(' expr ')'`
    /// using for atoms in rule group and PEG choice
    fn parse_or(&self, scanner: &mut impl Scanner, pegs: &[Peg]) -> Option<ResultParse> {
        let mut acc_error: Option<ParseError> = None;

        for atom in pegs {
            match self.parse_peg(scanner, atom) {
                Ok(node) => return Some(Ok(node)),
                Err(err) => {
                    acc_error = Some(match acc_error {
                        Some(err0) => err0.union(err),
                        None => err,
                    })
                }
            }
        }
        acc_error.map(Err)
    }

    /// parse a sequence of PEG atoms, e.g. `expr = 'a' 'b' '(' expr ')'`
    /// using for atoms in rule group and PEG sequence

    fn parse_rule_group(&self, scanner: &mut impl Scanner, r_group: &RuleGroup) -> ResultParse {
        self.parse_or(scanner, &r_group.atoms)
            .unwrap_or_else(|| unimplemented!("Only PEG rules are implemented"))
    }

    /// parse a rule by name, e.g. `expr`
    /// fail parsing fails according to the rule's PEG or Pratt definitions
    /// can pannic if the rule is not found,
    pub fn parse(&self, scanner: &mut impl Scanner, rule_name: &str) -> Result<Cst, Error> {
        let r_group = self.rules.get(rule_name).unwrap();
        self.parse_rule_group(scanner, r_group)
            .map_err(|err| err.into())
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

    #[test]
    fn parse_ref_rule() {
        let parser = Parser::default().add_peg("expr", Peg::Literal(Literal::from("abc")));
        let mut scanner = StringScanner::from("abcdef");

        {
            let result = parser.parse_ref_rule(&mut scanner, &RefRule::from("expr"));
            assert!(result.is_ok());

            let cst = result.unwrap();
            let theme = Theme::default();
            let cst_str = cst.make_string(&theme);
            assert_eq!(cst_str, "abc");
        }

        {
            let result = parser.parse_ref_rule(&mut scanner, &RefRule::from("expr"));
            let err = result.err().unwrap();

            let err_expected = ParseError::new(3, Peg::RefRule(RefRule::from("expr")));
            assert_eq!(err, err_expected);
        }
    }

    #[test]
    fn parse_choice() {
        let pegs = vec![
            Peg::Literal(Literal::from("abc")),
            Peg::Literal(Literal::from("def")),
        ];
        let parser = Parser::default();
        let mut scanner = StringScanner::from("abcdef");
        let theme = Theme::default();

        {
            let result = parser.parse_choice(&mut scanner, &pegs);
            assert!(result.is_ok());

            let cst = result.unwrap();
            assert_eq!(cst.make_string(&theme), "abc");
        }

        {
            let result = parser.parse_choice(&mut scanner, &pegs);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().make_string(&theme), "def");
        }

        {
            let result = parser.parse_choice(&mut scanner, &pegs);
            assert!(result.is_err());
            let err_expectect = ParseError::new(6, Peg::Literal(Literal::from("abc")))
                .union(ParseError::new(6, Peg::Literal(Literal::from("def"))));
            assert_eq!(result.err().unwrap(), err_expectect);
        }
    }
}
