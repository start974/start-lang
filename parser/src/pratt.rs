use crate::Peg;

/// Represents a Pratt operator node with precedence and associativity.
#[derive(Debug)]
pub struct Pratt {
    /// Fixity of the operator: infix, prefix, or postfix
    pub fixity: Fixity,

    /// Precedence level; higher values bind tighter
    pub precedence: usize,
}

/// Operator fixity for Pratt parsing.
#[derive(Debug)]
pub enum Fixity {
    /// Infix operator, e.g. `a + b`.
    Infix {
        /// Operator symbol, e.g. `+` in `a + b`
        operator: String,
        /// Left operand
        left: Box<Peg>,
        /// Right operand
        right: Box<Peg>,
        /// Associativity of the operator: left, right, or non-associative
        associativity: Assoc,
    },
    /// Prefix operator, e.g. `-a`.
    Prefix {
        /// Operator symbol, e.g. `-` in `-a`
        operator: String,
        /// Operand
        operand: Box<Peg>,
    },
    /// Postfix operator, e.g. `a!`.
    Postfix {
        /// Operator symbol, e.g. `!` in `a!`
        operator: String,
        /// Operand
        operand: Box<Peg>,
    },
}

/// Associativity of a Pratt operator.
#[derive(Debug, Clone, Copy)]
pub enum Assoc {
    /// Left-associative, e.g. `a + b + c` is parsed as `(a + b) + c`
    Left,
    /// Right-associative, e.g. `a ^ b ^ c` is parsed as `a ^ (b ^ c)`
    Right,
    /// Non-associative, e.g. `a == b == c` is not allowed
    NonAssoc,
}
