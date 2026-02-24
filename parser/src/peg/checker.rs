/*    /// check if a rule is nullable, i.e. can match the empty string*/
/*/// if a rule ref not exists suppose this rule is not nullable*/
/*fn nullable(&self, rule: &Peg) -> bool {*/
/*use peg::Peg::*;*/
/*match rule {*/
/*Literal(s) => s.is_empty(),*/
/*Class(_) => false,*/
/*RefRule(ref_rule) => self*/
/*.rules*/
/*.get(&ref_rule.name)*/
/*.is_some_and(|rules| rules.atom.iter().any(|r| self.nullable(r))),*/
/*Seq(pegs) => pegs.iter().all(|r| self.nullable(r)),*/
/*Choice(pegs) => pegs.iter().any(|r| self.nullable(r)),*/
/*Repeat(rep) => rep.min() == 0,*/
/*NegativeLookahead(_) | PositiveLookahead(_) => true,*/
/*}*/
/*}*/

/*fn check_peg(&self, rule: &Peg) -> Result<(), Errors> {*/
/*use peg::Peg::*;*/
/*match rule {*/
/*Literal(_) | Class(_) => Ok(()),*/
/*RefRule(ref_rule) => {*/
/*let name = &ref_rule.name;*/
/*if self.rule_exist(name) {*/
/*Ok(())*/
/*} else {*/
/*Err(error::not_defined(ref_rule).into())*/
/*}*/
/*}*/
/*Seq(pegs) | Choice(pegs) =>*/
/*{*/
/*#[allow(clippy::manual_try_fold)]*/
/*pegs.iter()*/
/*.fold(Ok(()), |acc, x| acc.combine(self.check_peg(x)))*/
/*}*/
/*Repeat(rep) => {*/
/*self.check_peg(rep.rule())*/
/*.combine(*/
/*if rep.max().is_none() && self.nullable(rep.rule()) {*/
/*Err(error::nullable_repetition(rep.rule()).into())*/
/*} else {*/
/*Ok(())*/
/*})*/
/*},*/
/*NegativeLookahead(peg) | PositiveLookahead(peg) => self.check_peg(peg),*/
/*}*/
/*}*/
