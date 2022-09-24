
use nom_locate::LocatedSpan;
use num_bigint::{BigInt, BigUint};
use num_rational::BigRational;
use std::{collections::HashMap, vec::Vec};
use super::parser_result::PInput;

type DataSpan = LocatedSpan<String, String>;

#[derive(Debug, Clone)]
pub struct Position<T> {
    span: DataSpan,
    content: Box<T>,
}

impl<T: Clone> Position<T> {
    pub fn new<'a>(s: &'a PInput<'a>, a: &T) -> Self {
        Position {
            span: DataSpan::new_extra(s.fragment().to_string(), "unknow".to_owned()),
            content: Box::new(a.clone()),
        }
    }
    pub fn unkown(a: T) -> Self {
        Position {
            span: DataSpan::new_extra(String::new(), "unknow".to_owned()),
            content: Box::new(a),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConstantCont {
    Nat(BigUint),
    Int(BigInt),
    Rat(BigRational),
    // Char(char),
    // Array(Vec<Constant>)
}
type Constant = Position<ConstantCont>;

#[derive(Debug, Clone)]
pub enum PatternCont {
    WillCard,
    Ident(String),
}
type Pattern = Position<PatternCont>;

#[derive(Debug, Clone)]
pub enum Abs {
    Fun(Vec<Pattern>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum ExprCont {
    Const(Box<Constant>),
    Var(String),
    Abst(Box<Abs>),
    App(Vec<Expr>),
}

pub type Expr = Position<ExprCont>;

pub struct Env {
    exprs: HashMap<String, Expr>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            exprs: HashMap::new(),
        }
    }
    pub fn add_expr(mut self, e: Expr) {
        self.exprs.insert(e.span.fragment().to_string(), e);
    }
}
