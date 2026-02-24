use errors::Error;
use location::SetSpan as _;

use crate::cst::*;
use crate::error::ParseError;
use crate::peg::*;
use crate::scanner::{Mark as _, Scanner};
use std::collections::{BTreeMap, HashMap};

type RuleGroup = BTreeMap<u8, Vec<Peg>>;

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

    /// parse end of input
    fn parse_eoi(&self, scanner: &mut impl Scanner, eoi: &Eoi) -> Result<(), ParseError> {
        if scanner.is_eoi() {
            Ok(())
        } else {
            let start = scanner.checkpoint();
            Err(error_expected(scanner, start, eoi.clone().into()))
        }
    }

    /// parse any, e.g. `.`
    fn parse_any(&self, scanner: &mut impl Scanner, any: &Any) -> Result<Token, ParseError> {
        let start = scanner.checkpoint();
        scanner
            .next()
            .map(|c| c.to_string())
            .map(|s| Token::from(s.as_str()).with_span(scanner.span_from(start)))
            .ok_or_else(|| error_expected(scanner, start, any.clone().into()))
    }

    /// parse a literal, e.g. `'a'`
    fn parse_literal(
        &self,
        scanner: &mut impl Scanner,
        literal: &Literal,
    ) -> Result<Token, ParseError> {
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
            .map(|()| Token::from(value.as_str()).with_span(scanner.span_from(start)))
    }

    /// parse class, e.g. `[a-z]`
    fn parse_class(&self, scanner: &mut impl Scanner, class: &Class) -> Result<Token, ParseError> {
        scanner
            .consume_if(|c| class.is_match(c))
            .map(|c| c.to_string())
            .map(|s| Token::from(s.as_str()).with_span(scanner.span_from(scanner.checkpoint())))
            .ok_or_else(|| error_expected(scanner, scanner.checkpoint(), class.clone().into()))
    }

    /// parse reference to another rule, e.g. `expr`
    fn parse_ref_rule(
        &self,
        scanner: &mut impl Scanner,
        ref_rule: &RefRule,
    ) -> Result<Cst, ParseError> {
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
            .map(|cst| cst.named(name))
            .map_err(|err| {
                if err.position() == start.position() {
                    error_expected(scanner, start, ref_rule.clone().into())
                } else {
                    err
                }
            })
    }

    /// parse sequence, e.g. `expr = 'a' 'b'`
    fn parse_seq(&self, scanner: &mut impl Scanner, pegs: &[Peg]) -> Result<Sequence, ParseError> {
        let start = scanner.checkpoint();
        pegs.iter()
            .try_fold(Sequence::default(), |acc, peg| {
                self.parse_peg(scanner, peg).map(|cst| acc.append(cst))
            })
            .inspect_err(|_| scanner.rollback(start))
    }

    /// parse choice between multiple PEG rules, e.g. `= 'a' / 'b' / '(' expr ')'`
    fn parse_choice(&self, scanner: &mut impl Scanner, pegs: &[Peg]) -> Result<Cst, ParseError> {
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

    /// parse optional PEG rule, e.g. `a?`
    fn parse_optional(&self, scanner: &mut impl Scanner, peg: &Peg) -> Option<Cst> {
        self.parse_peg(scanner, peg).ok()
    }

    /// parse repetition of a PEG rule
    fn parse_repetition(
        &self,
        scanner: &mut impl Scanner,
        peg: &Peg,
        repeat: &Repeat,
    ) -> Result<Sequence, ParseError> {
        let start = scanner.checkpoint();
        // parse minimal repetition
        let cst = repeat
            .iter_min()
            .try_fold(Sequence::default(), |acc, ()| {
                self.parse_peg(scanner, peg).map(|cst| acc.append(cst))
            })
            .inspect_err(|_| scanner.rollback(start))?;

        // parse maximal repetion
        repeat
            .iter_max()
            .try_fold(cst, |acc, ()| match self.parse_optional(scanner, peg) {
                Some(cst) => Ok(acc.append(cst)),
                None => Err(acc),
            })
            .or_else(|cst| Ok(cst))
    }

    /// parse lookead, e.g. `&a` `!b`
    fn parse_lookahead(
        &self,
        scanner: &mut impl Scanner,
        peg: &Peg,
        positive: bool,
    ) -> Result<(), ParseError> {
        let start = scanner.checkpoint();
        match (self.parse_peg(scanner, peg), positive) {
            (Ok(_), true) | (Err(_), false) => {
                scanner.rollback(start);
                Ok(())
            }
            (Err(e), true) => {
                scanner.rollback(start);
                Err(e)
            }
            (Ok(_), false) => Err(error_expected(
                scanner,
                start,
                Peg::negative_lookahead(peg.clone()),
            )),
        }
    }

    /// parse a PEG rule, e.g. `expr = 'a' / 'b'`
    fn parse_peg(&self, scanner: &mut impl Scanner, peg: &Peg) -> Result<Cst, ParseError> {
        match peg {
            Peg::Eoi(eoi) => self.parse_eoi(scanner, eoi).map(|()| Cst::nil()),
            Peg::Any(any) => self.parse_any(scanner, any).map(Cst::from),
            Peg::Literal(lit) => self.parse_literal(scanner, lit).map(Cst::from),
            Peg::Class(class) => self.parse_class(scanner, class).map(Cst::from),
            Peg::RefRule(rule_name) => self.parse_ref_rule(scanner, rule_name),
            Peg::Sequence(pegs) => self.parse_seq(scanner, pegs).map(Cst::from),
            Peg::Choice(pegs) => self.parse_choice(scanner, pegs),
            Peg::Optional(peg) => Ok(self.parse_optional(scanner, peg).into()),
            Peg::Repetition(peg, repeat) => {
                self.parse_repetition(scanner, peg, repeat).map(Cst::from)
            }
            Peg::Lookahead(peg, positive) => self
                .parse_lookahead(scanner, peg, *positive)
                .map(|()| Cst::nil()),
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
    fn parse_eoi() {
        let parser = Parser::default();
        let mut scanner = StringScanner::from("a");
        let peg = Peg::eoi();
        let theme = Theme::default();

        {
            let err = parser.parse_peg(&mut scanner, &peg).err().unwrap();
            let err_expected = ParseError::new(0, peg.clone());
            assert_eq!(err, err_expected);
        }
        {
            parser.parse_peg(&mut scanner, &Peg::any()).unwrap();
            let cst = parser.parse_peg(&mut scanner, &peg).unwrap();
            assert_eq!(cst.make_string(&theme), "");
        }
    }

    #[test]
    fn parse_any() {
        let parser = Parser::default();
        let mut scanner = StringScanner::from("ab");
        let peg = Peg::any();
        let theme = Theme::default();

        {
            let cst = parser.parse_peg(&mut scanner, &peg).unwrap();
            assert_eq!(cst.make_string(&theme), "a");
        }
        {
            let cst = parser.parse_peg(&mut scanner, &peg).unwrap();
            assert_eq!(cst.make_string(&theme), "b");
        }
        {
            let err = parser.parse_peg(&mut scanner, &peg).err().unwrap();
            let err_expected = ParseError::new(2, peg);
            assert_eq!(err, err_expected);
        }
    }

    #[test]
    fn parse_literal() {
        let parser = Parser::default();
        let mut scanner = StringScanner::from("abcdef");
        let peg = Peg::Literal(Literal::from("abc"));
        let theme = Theme::default();

        {
            let cst = parser.parse_peg(&mut scanner, &peg).unwrap();
            assert_eq!(cst.make_string(&theme), "abc");
        }

        {
            let result = parser.parse_peg(&mut scanner, &peg);
            let err = result.err().unwrap();

            let err_expected = ParseError::new(3, peg);
            assert_eq!(err, err_expected);
        }
    }

    #[test]
    fn parse_class() {
        let parser = Parser::default();
        let mut scanner = StringScanner::from("a1");
        let peg = Peg::Class(Class::from('a'..='z'));
        let theme = Theme::default();

        {
            let cst = parser.parse_peg(&mut scanner, &peg).unwrap();
            assert_eq!(cst.make_string(&theme), "a");
        }

        {
            let err = parser.parse_peg(&mut scanner, &peg).err().unwrap();
            let err_expected = ParseError::new(1, peg);
            assert_eq!(err, err_expected);
        }
    }

    #[test]
    fn parse_ref_rule_simple() {
        let parser = Parser::default().add_rule("expr", 0, Peg::Literal(Literal::from("abc")));
        let peg = Peg::RefRule(RefRule::from("expr"));
        let mut scanner = StringScanner::from("abcdef");
        let theme = Theme::default();

        {
            let cst = parser.parse_peg(&mut scanner, &peg).unwrap();
            assert_eq!(cst.make_string(&theme), "abc");
        }

        {
            let result = parser.parse_peg(&mut scanner, &peg);
            let err = result.err().unwrap();

            let err_expected = ParseError::new(3, peg);
            assert_eq!(err, err_expected);
        }
    }

    #[test]
    fn parse_seq() {
        let peg = Peg::Sequence(vec![
            Literal::from("abc").into(),
            Literal::from("def").into(),
        ]);
        let parser = Parser::default();
        let mut scanner = StringScanner::from("abcdefabc");
        let theme = Theme::default();

        {
            let cst = parser.parse_peg(&mut scanner, &peg).unwrap();
            assert_eq!(cst.make_string(&theme), "abcdef");
        }

        {
            let err = parser.parse_peg(&mut scanner, &peg).err().unwrap();
            let err_expected = ParseError::new(9, Peg::Literal(Literal::from("def")));
            assert_eq!(err, err_expected);
            assert_eq!(scanner.checkpoint().position(), 6);
        }
    }

    #[test]
    fn parse_choice() {
        let peg = Peg::Choice(vec![
            Literal::from("abc").into(),
            Literal::from("def").into(),
        ]);
        let parser = Parser::default();
        let mut scanner = StringScanner::from("abcdef");
        let theme = Theme::default();

        {
            let cst = parser.parse_peg(&mut scanner, &peg).unwrap();
            assert_eq!(cst.make_string(&theme), "abc");
        }

        {
            let cst = parser.parse_peg(&mut scanner, &peg).unwrap();
            assert_eq!(cst.make_string(&theme), "def");
        }

        {
            let err = parser.parse_peg(&mut scanner, &peg).err().unwrap();
            let err_expectect = ParseError::new(6, Peg::Literal(Literal::from("abc")))
                .union(ParseError::new(6, Peg::Literal(Literal::from("def"))));
            assert_eq!(err, err_expectect);
        }
    }

    #[test]
    fn parse_optional() {
        let peg = Peg::from(Literal::from("a")).optional();
        let parser = Parser::default();
        let mut scanner = StringScanner::from("ab");
        let theme = Theme::default();

        {
            let cst = parser.parse_peg(&mut scanner, &peg).unwrap();
            assert_eq!(cst.make_string(&theme), "a");
        }
        {
            let cst = parser.parse_peg(&mut scanner, &peg).unwrap();
            assert_eq!(cst.make_string(&theme), "");
        }
    }

    #[test]
    fn parse_zero_or_more() {
        let peg = Peg::from(Literal::from("a")).repetition(Repeat::zero_or_more());
        let parser = Parser::default();
        let mut scanner = StringScanner::from("ab");
        let theme = Theme::default();

        {
            let cst = parser.parse_peg(&mut scanner, &peg).unwrap();
            assert_eq!(cst.make_string(&theme), "a");
        }
        {
            let cst = parser.parse_peg(&mut scanner, &peg).unwrap();
            assert_eq!(cst.make_string(&theme), "");
        }
    }

    #[test]
    fn parse_one_or_more() {
        let peg = Peg::from(Literal::from("a")).repetition(Repeat::one_or_more());
        let parser = Parser::default();
        let mut scanner = StringScanner::from("aaaab");
        let theme = Theme::default();

        {
            let cst = parser.parse_peg(&mut scanner, &peg).unwrap();
            assert_eq!(cst.make_string(&theme), "aaaa");
        }
        {
            let err = parser.parse_peg(&mut scanner, &peg).err().unwrap();
            let err_expected = ParseError::new(4, Peg::Literal(Literal::from("a")));
            assert_eq!(err, err_expected);
        }
    }

    #[test]
    fn parse_exactly() {
        let peg = Peg::from(Literal::from("a")).repetition(Repeat::exactly(3));
        let parser = Parser::default();
        let mut scanner = StringScanner::from("aaaab");
        let theme = Theme::default();

        {
            let cst = parser.parse_peg(&mut scanner, &peg).unwrap();
            assert_eq!(cst.make_string(&theme), "aaa");
        }
        {
            let err = parser.parse_peg(&mut scanner, &peg).err().unwrap();
            let err_expected = ParseError::new(4, Peg::Literal(Literal::from("a")));
            assert_eq!(err, err_expected);
        }
    }

    #[test]
    fn parse_between() {
        let peg = Peg::from(Literal::from("a")).repetition(Repeat::between(2, 4));
        let parser = Parser::default();
        let mut scanner = StringScanner::from("aaaab");
        let theme = Theme::default();

        {
            let cst = parser.parse_peg(&mut scanner, &peg).unwrap();
            assert_eq!(cst.make_string(&theme), "aaaa");
        }
        {
            let err = parser.parse_peg(&mut scanner, &peg).err().unwrap();
            let err_expected = ParseError::new(4, Peg::Literal(Literal::from("a")));
            assert_eq!(err, err_expected);
        }
    }

    #[test]
    fn parse_positive_lookahead() {
        let lookead_lit: Peg = Literal::from("b").into();
        let peg = Peg::Sequence(vec![
            Peg::any(),
            Peg::positive_lookahead(lookead_lit.clone()),
        ]);
        let parser = Parser::default();
        let mut scanner = StringScanner::from("abbc");
        let theme = Theme::default();

        {
            let cst = parser.parse_peg(&mut scanner, &peg).unwrap();
            assert_eq!(cst.make_string(&theme), "a");
        }
        {
            let cst = parser.parse_peg(&mut scanner, &peg).unwrap();
            assert_eq!(cst.make_string(&theme), "b");
        }
        {
            let err = parser.parse_peg(&mut scanner, &peg).err().unwrap();
            let err_expected = ParseError::new(3, lookead_lit.clone());
            assert_eq!(err_expected, err);
        }
    }

    #[test]
    fn parse_negative_lookahead() {
        let lookead = Peg::negative_lookahead(Literal::from("b").into());
        let peg = Peg::Sequence(vec![Peg::any(), lookead.clone()]);
        let parser = Parser::default();
        let mut scanner = StringScanner::from("aabc");
        let theme = Theme::default();

        {
            let cst = parser.parse_peg(&mut scanner, &peg).unwrap();
            assert_eq!(cst.make_string(&theme), "a");
        }
        {
            let err = parser.parse_peg(&mut scanner, &peg).err().unwrap();
            let err_expected = ParseError::new(2, lookead.clone());
            assert_eq!(err_expected, err);
        }
    }
}
