use num_bigint::BigInt;

use super::{operator::Operator, token::Token};

pub enum Expr {
    Leaf(BigInt),
    UnaryOperator {
        op: Operator,
        rhs: Box<Expr>,
    },
    BinaryOperator {
        op: Operator,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

impl Expr {
    pub fn from(tokens: Vec<Token>) -> Self {
        todo!()
    }

    pub fn evaluate(&self) -> anyhow::Result<BigInt> {
        todo!()
    }
}
