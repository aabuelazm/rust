use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a, 'b>(word: &'a str, possible_anagrams: &[&'b str]) -> HashSet<&'b str> {
    let mut output: HashSet<&'b str> = HashSet::new();
    let input: String = word.to_lowercase();

    for possible_match in possible_anagrams {
        if is_anagram(input.clone(), possible_match.to_lowercase().clone()) {
            output.insert(possible_match);
        }
    }

    output
}

fn is_anagram(input: String, mut possible: String) -> bool {
    if input.len() == possible.len() {
        if input.to_owned() != possible {
            for c in input.grapheme_indices(true) {
                match possible.find(c.1) {
                    Some(position) => {
                        possible.remove(position);
                    }
                    None => {
                        return false;
                    }
                }
            }
            return true;
        }
        return false;
    } else {
        false
    }
}
