use num_bigint::BigInt;

use super::operator::OPERATORS;

pub struct NumericBase {
    base: String,
}

impl NumericBase {
    pub fn from_base(base: &str) -> anyhow::Result<Self> {
        let base = base.trim();

        if base.is_empty() {
            anyhow::bail!("Invalid base: must have at least one symbol");
        }
        if base.is_empty() {
            anyhow::bail!("Invalid base: must have at least one symbol");
        }
        if let Some((index_1, index_2, character)) = find_duplicate_char(base) {
            anyhow::bail!(
                "Invalid base: character '{}' (U+{:0>4X}) repeats at index {} and {}",
                character,
                character as usize,
                index_1,
                index_2
            );
        }
        if let Some(character) = find_conflict_with_operator(base) {
            anyhow::bail!(
                "Invalid base: character '{}' (U+{:0>4X}) conflicts with operator of same symbol",
                character,
                character as usize,
            );
        }

        Ok(Self {
            base: base.to_owned(),
        })
    }

    pub fn get_digit_value(&self, c: char) -> Option<usize> {
        self.base.find(c)
    }
    pub fn get_base_length(&self) -> usize {
        self.base.len()
    }
}

fn find_duplicate_char(s: &str) -> Option<(usize, usize, char)> {
    s.chars().enumerate().find_map(|(i, c)| {
        s.chars()
            .enumerate()
            .skip(i + 1)
            .find(|(_, other)| c == *other)
            .map(|(j, _)| (i, j, c))
    })
}

fn find_conflict_with_operator(s: &str) -> Option<char> {
    for (op_character, _) in OPERATORS.iter().flatten() {
        if s.contains(*op_character) {
            return Some(*op_character);
        }
    }
    None
}
