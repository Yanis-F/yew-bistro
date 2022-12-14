use num_bigint::BigInt;

use super::{
    numeric_base::NumericBase,
    operator::{Operator, OPERATORS},
};

pub struct ParseResult<T> {
    pub characters_read: usize,
    pub value: T,
}

pub fn parse_number(numeric_base: &NumericBase, s: &str) -> Option<ParseResult<BigInt>> {
    let digits: Vec<usize> = s
        .chars()
        .map_while(|c| numeric_base.get_digit_value(c))
        .collect();

    if digits.is_empty() {
        None
    } else {
        Some(ParseResult {
            characters_read: digits.len(),
            value: digits.into_iter().fold(BigInt::from(0), |a, b| {
                a * numeric_base.get_base_length() + b
            }),
        })
    }
}

pub fn parse_operator(s: &str) -> Option<ParseResult<Operator>> {
    for (operator_character, operator) in OPERATORS.iter().flatten() {
        if s.starts_with(*operator_character) {
            return Some(ParseResult {
                characters_read: 1,
                value: *operator,
            });
        }
    }
    return None;
}
