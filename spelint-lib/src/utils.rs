use std::iter::repeat;

pub fn match_str_case(pattern: &str, value: &str) -> String {
    let last_char = pattern.chars().last().unwrap_or_default();

    pattern
        .chars()
        .chain(repeat(last_char))
        .zip(value.chars())
        .map(|(p, c)| match_char_case(p, c))
        .collect()
}

fn match_char_case(pattern: char, value: char) -> String {
    match pattern.is_uppercase() {
        true => value.to_uppercase().to_string(),
        false => value.to_lowercase().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::match_str_case;

    #[test]
    fn test_case_match() {
        let original = "sPoNgEcAsE";
        let value = "SPOngeCaSE";

        assert_eq!(original, match_str_case(original, value));
    }
}
