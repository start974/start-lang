use crate::frontend::lexer::utils::{Parse, TokenResult};
use crate::frontend::lexer::Lexer;

#[derive(Clone, Debug)]
pub struct NotationGroup {
    name: String,
    rules: Vec<String>,
}

impl NotationGroup {
    pub fn new(name: &str, rules: &Vec<String>) -> Self {
        assert!(!rules.is_empty(), "rules is empty");
        NotationGroup {
            name: name.to_string(),
            rules: rules.clone(),
        }
    }

    pub fn name(&mut self) -> String {
        self.name.clone()
    }

    pub fn rules(&mut self) -> Vec<String> {
        self.rules.clone()
    }

    // fn parse_group_rule(env: NotationEnv, s: Span) -> IResult<String, NotationRule> {
    //     let s, rule_name = self.parse_name(s)?;
    //     let rule = env.get_rule(rule_name)?;
    //     Ok((s, rule))
    // }

    // fn parse_nota_rule(_env: NotationEnv, s: Span) -> IResult<String, NotationRule> {
    //     let s, _ = tag("Rule")(s)?;
    //     let s, _ = multispace1(s)?;
    // }
}

/*
* Notation group grammar:
* <group>      := "Group" <name> ":=" <nota-group>
* <rule>       := "Rule" ":" <rule-exp> ":=" <exp>
*/
impl<'l> Parse<'l> for NotationGroup {
    fn parse(lexer: &mut Lexer) -> TokenResult<Box<Self>> {
        lexer.next_token().tag("Group")?;
        let rules = vec!["test1".to_string()];
        Ok(Box::new(Self::new("test", &rules)))
        // let (s, _) = multispace1(s)?;
        // let (s, name) = parse_name(env, s)?;
        // let (s, _) = multispace1(s)?;
        // let (s, _) = tag(":=")(s)?;
        // let (s, _) = multispace1(s)?;
        // let (s, opt_res = opt(|s| -> {
        //     let s, _ = tag("...")
        //     let s, _ = multispace0(s)?;
        //     let s, _ = char(s)?;
        //     multispace0(s)
        // })(s);
        // let mut rules = match opt_res {
        //     None -> vec![],
        //     Some _ -> env.get_group(name)?,
        // };
        // let s, rule0 = self.parse_rule(s)?;
        // rules.push(rule0);
        // let s, rules1 = many0( |s| -> {
        //     let s, _ = multispace0(s)?;
        //     let s, _ = char('|')?;
        //     self.parse_group(s)
        // })(s);
        // rules.append(rules1);
        // let res = NotationGroup{name, rules};
        // Ok((s, res))
    }
}
