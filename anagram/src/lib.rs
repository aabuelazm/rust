use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
    let mut output = HashSet::new();
    let input: &str = word.chars().map(|c| c.to_lowercase());
    
    for possible_match in possible_anagrams {
         if is_anagram(input, possible_match.to_owned()) {
             output.insert(*possible_match);
        }
    }

    output
}

fn is_anagram(input: String, possible: String) -> bool {
    unimplemented!("Just wait")
}
