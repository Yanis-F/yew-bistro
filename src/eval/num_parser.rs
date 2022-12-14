pub struct NumParser {
    base: String,
}

impl NumParser {
    pub fn from_base(base: &str) -> anyhow::Result<Self> {
        if let Some((index_1, index_2, character)) = find_duplicate_char(&base) {
            anyhow::bail!(
                "Invalid base: character '{}' (U+{:0>4X}) repeats at index {} and {}",
                character,
                character as usize,
                index_1,
                index_2
            );
        }

        Ok(Self { base: base.to_owned() })
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
