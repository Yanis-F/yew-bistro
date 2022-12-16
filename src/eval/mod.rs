mod numeric_base;
mod operator;
mod parser;
mod token;

use std::collections::LinkedList;

use num_bigint::BigInt;

use self::{operator::Operator, token::Token};
use crate::eval::{numeric_base::NumericBase, operator::OPERATORS, token::tokenize_strexpr};

pub fn evaluate(expr_str: &str, base_str: &str) -> anyhow::Result<String> {
    log::info!("evaluating '{}'", expr_str);

    let numeric_base = NumericBase::from_base(base_str)?;
    let tokens = tokenize_strexpr(expr_str, &numeric_base)?;

    let result = evaluate_tokens(tokens)?;
    Ok(result.to_string())
}

fn evaluate_tokens(tokens: Vec<Token>) -> anyhow::Result<BigInt> {
    log::debug!("Tokens are {:#?}", tokens);

    let mut tokens = tokens;
    tokens = evaluate_parenthesis(tokens)?;
    log::debug!("After parenthesis reduction: {:#?}", tokens);

    tokens = evaluate_unary_operators(tokens);
    log::debug!("After unary operators reduction: {:#?}", tokens);

    for operator_level_of_priority in OPERATORS.iter() {
        let operator_vec: Vec<Operator> = operator_level_of_priority
            .values()
            .into_iter()
            .copied()
            .collect();
        tokens = evaluate_binary_operators(tokens, &operator_vec)?;
        log::debug!(
            "After binary operators reduction of op '{:?}': {:#?}",
            operator_vec,
            tokens
        );
    }

	assert!(tokens.len() == 1);
	if let Token::Number(result) = tokens.into_iter().next().unwrap() {
		Ok(result)
	} else {
		unreachable!()
	}
}

fn evaluate_parenthesis(token_vec: Vec<Token>) -> anyhow::Result<Vec<Token>> {
    let mut new_tokens = Vec::new();

    for token in token_vec {
        if let Token::TokenList(tokenlist) = token {
            if let Some(Token::Number(_)) = new_tokens.last() {
                new_tokens.push(Token::Operator(Operator::Multiplication))
            }
            new_tokens.push(Token::Number(evaluate_tokens(tokenlist)?))
        } else {
			new_tokens.push(token);
		}
    }

    Ok(new_tokens)
}

fn evaluate_unary_operators(token_vec: Vec<Token>) -> Vec<Token> {
    let mut new_tokens = Vec::new();

    let mut negate_next_number = false;
    for token in token_vec {
        if let Token::Number(value) = token {
            new_tokens.push(Token::Number(if negate_next_number {
                -value
            } else {
                value
            }));
            negate_next_number = false;
        } else if let Token::Operator(op) = token {
            if let Some(Token::Operator(_)) = new_tokens.last() {
                match op {
                    Operator::Addition => continue,
                    Operator::Subtraction => {
                        negate_next_number = !negate_next_number;
                        continue;
                    }
                    _ => {}
                }
            }
            new_tokens.push(Token::Operator(op))
        } else {
            unreachable!("we're already cleared the parenthesis")
        }
    }

    new_tokens
}

fn evaluate_binary_operators(
    token_vec: Vec<Token>,
    operators_to_evaluate: &[Operator],
) -> anyhow::Result<Vec<Token>> {
    let mut new_tokens = Vec::new();

	let mut token_iter = token_vec.into_iter();
	let mut lhs =
	if let Some(Token::Number(v)) = token_iter.next() {
		v
	} else {
		anyhow::bail!("Logic error: Expression begins with a binary operator. it lacks a left-hand-side value");
	};

    while let Some(next_operator) = token_iter.next() {
		if let Token::Operator(op) = next_operator {
			if operators_to_evaluate.contains(&op) {
				if let Some(Token::Number(rhs)) = token_iter.next() {
					lhs = get_op_result(lhs, op, rhs)?;
				} else {
					anyhow::bail!(
						"Logic error: binary operator does not have a number to its right"
					);
				}
			} else {
				log::debug!("Does not contain, {:?}", new_tokens);
				new_tokens.push(Token::Number(lhs));
				new_tokens.push(Token::Operator(op));
				log::debug!("after pushed the non-handled op: {:?}", new_tokens);
				if let Some(Token::Number(v)) = token_iter.next() {
					lhs = v;
				} else {
					anyhow::bail!(
						"Logic error: binary operator does not have a number to its right"
					);
				}
			}
		} else {
			anyhow::bail!("Logic error: multiple consecutive numbers without operators");
		}
    }

	log::debug!("pushing the final value {} onto {:?}", lhs, new_tokens);
    new_tokens.push(Token::Number(lhs));
	log::debug!("total:  {:?}", new_tokens);

    Ok(new_tokens)
}

fn get_op_result(lhs: BigInt, op: Operator, rhs: BigInt) -> anyhow::Result<BigInt> {
    Ok(match op {
        Operator::Power => {
            let mut result = lhs.clone();
            let mut counter = BigInt::from(0);
            while counter < rhs {
                result  *= lhs.clone();
                counter += 1;
            }
            result
        }
        Operator::Division => {
            if rhs == BigInt::from(0) {
                anyhow::bail!("Math error: Division by 0 when evaluating {}/0", lhs);
            }
            lhs / rhs
        }
        Operator::Multiplication => lhs * rhs,
        Operator::Addition => lhs + rhs,
        Operator::Subtraction => lhs - rhs,
    })
}
