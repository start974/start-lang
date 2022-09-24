use core::fmt;

type Var = String;

#[derive(Clone)]
pub enum Rep {
    Opt,
    OneMany,
    ZeroMany,
    Exact(u16),
    Between(u16, u16)
}

#[derive(Clone)]
pub enum Rule {
    Param(Var, Box<Rule>),
    Tag(String),
    Symbol(String),
    Alt(Box<Rule>, Box<Rule>),
    Concat(Box<Rule>, Box<Rule>),
    Next(Box<Rule>, Box<Rule>),
    List(String),
    Group(Box<Rule>),
    Rep(Box<Rule>, Rep),
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rule::Param(var, rule) => write!(f, "{var} = {rule}"),
            Rule::Tag(tag) => write!(f, "\"{tag}\""),
            Rule::Symbol(symb) => write!(f, "{symb}"),
            Rule::Alt(r_l, r_r) => write!(f, "{r_l} | {r_r}"),
            Rule::Concat(r_l, r_r) => write!(f, "{r_l}, {r_r}"),
            Rule::Next(r_l, r_r) => write!(f, "{r_l} {r_r}"),
            Rule::List(l) => write!(f, "[{l}]"),
            Rule::Group(r) => write!(f, "({r})"),
            Rule::Rep(r, Rep::Opt) => write!(f, "{r}?"),
            Rule::Rep(r, Rep::ZeroMany) => write!(f, "{r}*"),
            Rule::Rep(r, Rep::OneMany) => write!(f, "{r}+"),
            Rule::Rep(r, Rep::Exact(n)) => write!(f, "{r}{{{n}}}"),
            Rule::Rep(r, Rep::Between(n, m)) => write!(f, "{r}{{{n}, {m}}}"),
        }
    }
}