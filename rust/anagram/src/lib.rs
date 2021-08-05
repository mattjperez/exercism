use std::collections::HashSet;

/// Returns sorted vec of chars
fn sort_word(word: &str) -> Vec<char> {
    let mut word = word.chars().collect::<Vec<_>>();
    word.sort_unstable();
    word
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lc_word = word.to_lowercase();

    let sorted_lc_word = sort_word(&lc_word);

    possible_anagrams
        .iter()
        .filter(|&a| {
            let lc_anagram = a.to_lowercase();
            lc_anagram != lc_word && sorted_lc_word == sort_word(&lc_anagram)
        })
        .map(|x| *x)
        .collect()
}
