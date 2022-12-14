use num_bigint::BigInt;

use super::{numeric_base::NumericBase, operator::Operator, parser::*};

#[derive(Debug)]
pub enum Token {
    Number(BigInt),
    Operator(Operator),
    TokenList(Vec<Token>),
}

pub fn tokenize_strexpr(strexpr: &str, numeric_base: &NumericBase) -> anyhow::Result<Vec<Token>> {
    let mut strexpr = strexpr.to_owned();
    strexpr.retain(|c| !c.is_whitespace());
    let mut strexpr = strexpr.as_mut_str();

    let mut tokens = Vec::new();

    while !strexpr.is_empty() {
        let (token, character_read) = parse_next_token(strexpr, numeric_base)?;

        tokens.push(token);
        (_, strexpr) = strexpr.split_at_mut(character_read);
    }

    Ok(tokens)
}

fn parse_next_token(strexpr: &str, numeric_base: &NumericBase) -> anyhow::Result<(Token, usize)> {
    if let Some(res) = parse_number(numeric_base, strexpr) {
        Ok((Token::Number(res.value), res.characters_read))
    } else if let Some(res) = parse_operator(strexpr) {
        Ok((Token::Operator(res.value), res.characters_read))
    } else if let Some((sub_strexpr, character_read)) = extract_parenthesis_content(strexpr)? {
        let subtokens = tokenize_strexpr(&sub_strexpr, numeric_base)?;
        Ok((Token::TokenList(subtokens), character_read))
    } else {
        anyhow::bail!("Syntax error: unknown token at '{}'", strexpr);
    }
}

fn extract_parenthesis_content(strexpr: &str) -> anyhow::Result<Option<(String, usize)>> {
    let mut depth = 0;

    for (outbound_index, character) in strexpr.chars().enumerate() {
        match character {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        };

        if depth <= 0 {
            if outbound_index == 0 {
                return Ok(None);
            }
            let substring = strexpr[1..outbound_index].to_string();
            log::debug!("substring is '{}'", substring);
            return Ok(Some((substring, outbound_index + 1)));
        }
    }

    anyhow::bail!("Syntax error: unmatched parenthesis at '{}'", strexpr);
}
