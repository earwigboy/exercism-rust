use std::collections::HashSet;

fn sort(word: &str) -> String {
    let mut sorted_chars: Vec<char> = word.chars().collect();
    sorted_chars.sort_unstable();
    sorted_chars.into_iter().collect()
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let sorted_word = sort(&lowercase_word);

    possible_anagrams
        .iter()
        .filter(|x| x.len() == word.len())
        .filter(|x| {
            let lowercase_x = x.to_lowercase();
            lowercase_x != lowercase_word && sorted_word == sort(&lowercase_x)
        })
        .cloned()
        .collect()
}
