use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a, 'b>(word: &'a str, possible_anagrams: &[&'b str]) -> HashSet<&'b str> {
    let mut output: HashSet<&'b str> = HashSet::new();
    let input: String = word.to_lowercase();

    for possible_match in possible_anagrams {
        if is_anagram(&input, possible_match.to_lowercase()) {
            output.insert(possible_match);
        }
    }

    output
}

fn is_anagram(input: &String, possible: String) -> bool {
    let mut copy = possible.clone();

    if input.len() == copy.len() {
        if *input != copy {
            for c in input.graphemes(true) {
                match copy.find(c) {
                    Some(position) => {
                        copy.remove(position);
                    }
                    None => {
                        return false;
                    }
                }
                return true;
            }
        }
        return false;
    }
    false
}
