use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut my_anagrams: HashSet<&'a str> = HashSet::new();
    let word_l = word.to_lowercase();
    let mut word_sorted: Vec<char> = word_l.chars().collect();
    word_sorted.sort_unstable();
    for anagram in possible_anagrams {
        let anagram_l = anagram.to_lowercase();
        if word_l.len() == anagram_l.len() && word_l != anagram_l {
            let mut word2: Vec<char> = anagram_l.chars().collect();
            word2.sort_unstable();
            if word_sorted == word2 {
                my_anagrams.insert(anagram);
            }
        }
    }
    my_anagrams
}
