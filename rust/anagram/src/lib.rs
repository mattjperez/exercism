use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let word_lo = word.to_lowercase();
    let mut word_chars = word_lo.chars().collect::<Vec<char>>();
    word_chars.sort_unstable();
    for possible_anagram in possible_anagrams {
        let possible_anagram_lo = possible_anagram.to_lowercase();
        if word_lo != possible_anagram_lo {
            let mut possible_anagram_chars = possible_anagram_lo.chars().collect::<Vec<char>>();
            possible_anagram_chars.sort_unstable();
            if word_chars == possible_anagram_chars {
                anagrams.insert(*possible_anagram);
            }
        }
    }
    anagrams
}
