use maplit::hashmap;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
    Power,

    Multiplication,
    Division,

    Addition,
    Subtraction,
}

lazy_static! {
    pub static ref OPERATORS: Vec<HashMap<char, Operator>> = vec![
        hashmap! {
            '^' => Operator::Power
        },
        hashmap! {
            '*' => Operator::Multiplication,
            '/' => Operator::Division,
        },
        hashmap! {
            '+' => Operator::Addition,
            '-' => Operator::Subtraction,
        },
    ];
}
