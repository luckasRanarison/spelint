use crate::utils::match_str_case;

use fst::Map;
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
        Self {
            alphabet: alphabet.graphemes(true).map(str::to_string).collect(),
            dictionary: Map::from_iter(dictionary).unwrap(),
        }
    }

    pub fn check(&self, word: &str) -> bool {
        self.dictionary.contains_key(word.to_lowercase())
    }

    pub fn get_corrections(&self, word: &str, depth: usize, limit: usize) -> Vec<String> {
        let mut candidates = self
            .get_edits(&word.to_lowercase(), depth)
            .into_iter()
            .filter_map(|w| self.dictionary.get(&w).map(|freq| (freq, w)))
            .take(limit)
            .collect::<Vec<_>>();

        candidates.sort_by(|a, b| b.0.cmp(&a.0));

        candidates
            .into_iter()
            .map(|(_, s)| match_str_case(word, &s))
            .collect()
    }

    fn get_edits_one(&self, word: &str) -> HashSet<String> {
        let splits = word
            .grapheme_indices(true)
            .map(|(i, _)| (&word[..i], &word[i..]))
            .collect::<Vec<_>>();

        let deletions = splits
            .iter()
            .map(|(l, r)| format!("{l}{}", r.graphemes(true).skip(1).collect::<String>()));

        let insertions = splits
            .iter()
            .flat_map(|(l, r)| self.alphabet.iter().map(move |c| format!("{l}{c}{r}")));

        let replacements = splits.iter().flat_map(|(l, r)| {
            let r = r.graphemes(true).skip(1).collect::<String>();
            self.alphabet.iter().map(move |c| format!("{l}{c}{r}"))
        });

        let transpositions = splits.iter().flat_map(|(l, r)| {
            let g = r.graphemes(true).collect::<Vec<_>>();
            match g.get(..2) {
                Some([f, s]) => Some(format!("{l}{s}{f}{}", g[2..].join(""))),
                _ => None,
            }
        });

        deletions
            .chain(insertions)
            .chain(replacements)
            .chain(transpositions)
            .collect()
    }

    fn get_edits(&self, word: &str, depth: usize) -> HashSet<String> {
        match depth {
            0 => [word.to_string()].into(),
            _ => self
                .get_edits_one(word)
                .into_iter()
                .flat_map(|w| self.get_edits(&w, depth - 1))
                .collect(),
        }
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
