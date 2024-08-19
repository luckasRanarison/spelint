use crate::utils::match_str_case;

use fst::{automaton::Levenshtein, IntoStreamer, Map};
use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Default)]
pub struct SpellChecker {
    alphabet: HashSet<String>,
    dictionary: Map<Vec<u8>>,
}

impl SpellChecker {
    pub fn new<T>(alphabet: &str, dictionary: T) -> Self
    where
        T: IntoIterator<Item = (String, u64)>,
    {
        let mut dictionary = dictionary.into_iter().collect::<Vec<_>>();

        dictionary.sort_by(|a, b| a.0.cmp(&b.0));

        Self {
            alphabet: alphabet.graphemes(true).map(str::to_string).collect(),
            dictionary: Map::from_iter(dictionary).unwrap(),
        }
    }

    pub fn check(&self, word: &str) -> bool {
        self.dictionary.contains_key(word.to_lowercase())
    }

    pub fn get_corrections(&self, word: &str, depth: u32, limit: usize) -> Vec<String> {
        let candidates = Levenshtein::new(word, depth).unwrap();
        let stream = self.dictionary.search(candidates).into_stream();
        let mut corrections = stream.into_str_vec().unwrap();

        corrections.sort_by(|a, b| b.1.cmp(&a.1));

        corrections
            .into_iter()
            .take(limit)
            .map(|(s, _)| match_str_case(word, &s))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::SpellChecker;

    #[test]
    fn test_correction() {
        let alphabet = "hdelomopr";
        let dictionary = [
            ("hell".to_string(), 70),
            ("hello".to_string(), 100),
            ("help".to_string(), 90),
            ("lorem".to_string(), 20),
            ("world".to_string(), 100),
        ];

        let checker = SpellChecker::new(alphabet, dictionary);
        let suggestions = checker.get_corrections("hella", 1, 5);
        let expected = vec!["hello", "hell"];

        assert_eq!(expected, suggestions);

        let suggestions = checker.get_corrections("hella", 2, 5);
        let expected = vec!["hello", "help", "hell"];

        assert_eq!(expected, suggestions);
    }
}
