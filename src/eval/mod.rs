mod expr;
mod numeric_base;
mod operator;
mod parser;
mod token;

use crate::eval::{numeric_base::NumericBase, token::tokenize_strexpr};

pub fn evaluate(expr_str: &str, base_str: &str) -> anyhow::Result<String> {
    log::info!("evaluating '{}'", expr_str);

    let numeric_base = NumericBase::from_base(base_str)?;
    let tokens = tokenize_strexpr(expr_str, &numeric_base)?;

    log::debug!("Tokens are {:#?}", tokens);

    Ok("todo".to_string())
}
