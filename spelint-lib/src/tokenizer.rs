use regex::Regex;

#[derive(Debug)]
pub struct Token<'a> {
    pub byte_start: usize,
    pub byte_end: usize,
    pub text: &'a str,
}

#[derive(Debug)]
pub struct Tokenizer {
    word_regex: Regex,
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self {
            word_regex: Regex::new(r"[\p{Alphabetic}]+").unwrap(),
        }
    }
}

impl Tokenizer {
    pub fn tokenize<'a>(&self, text: &'a str) -> Vec<Token<'a>> {
        self.word_regex
            .find_iter(text)
            .map(|m| Token {
                byte_start: m.start(),
                byte_end: m.end(),
                text: m.as_str(),
            })
            .collect()
    }
}
