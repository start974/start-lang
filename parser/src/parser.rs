use errors::Error;

use crate::cst::Cst;
use crate::error::ParseError;
use crate::peg::{Class, Literal, Peg, RefRule};
use crate::{Mark as _, Scanner};
use std::collections::{BTreeMap, HashMap};

type RuleGroup = BTreeMap<u8, Vec<Peg>>;
type ResultParse = Result<Cst, ParseError>;

/// Full parser containing all grammar rules.
///
/// The parser supports:
/// - PEG rules (standard grammar)
/// - Pratt rules (precedence-based parsing)
///
/// Both are unified under the same rule table to avoid splitting the logic.
#[derive(Debug, Default)]
pub struct Parser(HashMap<String, RuleGroup>);
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
    pub fn add_rule(mut self, name: &str, prec: u8, rule: Peg) -> Self {
        self.0
            .entry(name.into())
            .or_default()
            .entry(prec)
            .or_default()
            .push(rule);

        self
    }

    /// get rule group by name
    fn get_group(&self, name: &str) -> &RuleGroup {
        self.0
            .get(name)
            .unwrap_or_else(|| panic!("Rule \"{name}\" not found in parser rules"))
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
        let start = scanner.checkpoint();

        // get choices with precedence
        let (_, choices) = {
            let r_group = self.get_group(name);
            match ref_rule.prec {
                // get rule at max precedence
                None => r_group.iter().next_back(),
                Some(prec) => r_group.range(..=prec).next_back(),
            }
            .expect("Rules cannot be empty")
        };

        self.parse_choice(scanner, choices)
            .map(|cst| cst.with_name(ref_rule.clone()))
            .map_err(|err| {
                if err.position() == start.position() {
                    error_expected(scanner, start, ref_rule.clone().into())
                } else {
                    err
                }
            })
    }

    /// parse sequence, e.g. `expr = 'a' 'b'`
    fn parse_seq(&self, scanner: &mut impl Scanner, pegs: &[Peg]) -> ResultParse {
        let start = scanner.checkpoint();
        pegs.iter()
            .try_fold(Cst::nil(), |acc, peg| {
                self.parse_peg(scanner, peg).map(|cst| acc.append(cst))
            })
            .inspect_err(|_| scanner.rollback(start))
    }

    /// parse choice between multiple PEG rules, e.g. `= 'a' / 'b' / '(' expr ')'`
    fn parse_choice(&self, scanner: &mut impl Scanner, pegs: &[Peg]) -> ResultParse {
        let mut acc_error: Option<ParseError> = None;

        for peg in pegs {
            match self.parse_peg(scanner, peg) {
                Ok(node) => return Ok(node),
                Err(err) => {
                    acc_error = Some(match acc_error {
                        Some(err0) => err0.union(err),
                        None => err,
                    })
                }
            }
        }
        acc_error.map(Err).expect("Rule or cannot be empty")
    }

    /// parse a PEG rule, e.g. `expr = 'a' / 'b'`
    fn parse_peg(&self, scanner: &mut impl Scanner, peg: &Peg) -> ResultParse {
        match peg {
            Peg::Literal(lit) => self.parse_literal(scanner, lit),
            Peg::Class(class) => self.parse_class(scanner, class),
            Peg::RefRule(rule_name) => self.parse_ref_rule(scanner, rule_name),
            Peg::Seq(pegs) => self.parse_seq(scanner, pegs),
            Peg::Choice(pegs) => self.parse_choice(scanner, pegs),
            _ => unimplemented!("Not implemented yet"),
        }
    }

    /// parse a rule by name, e.g. `expr`
    /// fail parsing fails according to the rule's PEG or Pratt definitions
    /// can pannic if the rule is not found,
    pub fn parse(&self, scanner: &mut impl Scanner, rule_name: &str) -> Result<Cst, Error> {
        let ref_rule = RefRule::from(rule_name);
        self.parse_ref_rule(scanner, &ref_rule)
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
    fn parse_ref_rule_simple() {
        let parser = Parser::default().add_rule("expr", 0, Peg::Literal(Literal::from("abc")));
        let mut scanner = StringScanner::from("abcdef");

        {
            let result = parser.parse_ref_rule(&mut scanner, &RefRule::from("expr"));

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
    fn parse_seq() {
        let pegs = vec![
            Peg::Literal(Literal::from("abc")),
            Peg::Literal(Literal::from("def")),
        ];
        let parser = Parser::default();
        let mut scanner = StringScanner::from("abcdefabc");
        let theme = Theme::default();

        {
            let result = parser.parse_seq(&mut scanner, &pegs);

            let cst = result.unwrap();
            let cst_str = cst.make_string(&theme);
            assert_eq!(cst_str, "abcdef");
        }

        {
            let result = parser.parse_seq(&mut scanner, &pegs);

            let err_expected = ParseError::new(9, Peg::Literal(Literal::from("def")));
            let err = result.err().unwrap();
            assert_eq!(err, err_expected);
            assert_eq!(scanner.checkpoint().position(), 6);
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

            let cst = result.unwrap();
            assert_eq!(cst.make_string(&theme), "abc");
        }

        {
            let result = parser.parse_choice(&mut scanner, &pegs);
            assert_eq!(result.unwrap().make_string(&theme), "def");
        }

        {
            let result = parser.parse_choice(&mut scanner, &pegs);
            let err_expectect = ParseError::new(6, Peg::Literal(Literal::from("abc")))
                .union(ParseError::new(6, Peg::Literal(Literal::from("def"))));
            assert_eq!(result.err().unwrap(), err_expectect);
        }
    }
}
