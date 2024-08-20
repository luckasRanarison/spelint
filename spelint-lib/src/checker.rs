use crate::{
    tokenizer::{Token, Tokenizer},
    utils::match_str_case,
};

use fst::{automaton::Levenshtein, IntoStreamer, Map};

#[derive(Debug, Default)]
pub struct SpellChecker {
    dictionary: Map<Vec<u8>>,
    tokenizer: Tokenizer,
}

impl SpellChecker {
    pub fn new<T>(dictionary: T) -> Self
    where
        T: IntoIterator<Item = (String, u64)>,
    {
        let mut dictionary = dictionary.into_iter().collect::<Vec<_>>();

        dictionary.sort_by(|a, b| a.0.cmp(&b.0));

        Self {
            dictionary: Map::from_iter(dictionary).unwrap(),
            tokenizer: Tokenizer::default(),
        }
    }

    pub fn get_unknowns<'a>(&self, sentence: &'a str) -> Vec<Token<'a>> {
        self.tokenizer
            .tokenize(sentence)
            .into_iter()
            .filter(|t| !self.check(t.text))
            .collect()
    }

    pub fn get_corrections(&self, word: &str, distance: u32, limit: usize) -> Vec<String> {
        let candidates = Levenshtein::new(word, distance).unwrap();
        let stream = self.dictionary.search(candidates).into_stream();
        let mut corrections = stream.into_str_vec().unwrap();

        corrections.sort_by(|a, b| b.1.cmp(&a.1));

        corrections
            .into_iter()
            .take(limit)
            .map(|(s, _)| match_str_case(word, &s))
            .collect()
    }

    fn check(&self, word: &str) -> bool {
        self.dictionary.contains_key(word.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::SpellChecker;

    #[test]
    fn test_correction() {
        let dictionary = [
            ("hell".to_string(), 70),
            ("hello".to_string(), 100),
            ("help".to_string(), 90),
            ("lorem".to_string(), 20),
            ("world".to_string(), 100),
        ];

        let checker = SpellChecker::new(dictionary);
        let suggestions = checker.get_corrections("hella", 1, 5);
        let expected = vec!["hello", "hell"];

        assert_eq!(expected, suggestions);

        let suggestions = checker.get_corrections("hella", 2, 5);
        let expected = vec!["hello", "help", "hell"];

        assert_eq!(expected, suggestions);
    }
}
