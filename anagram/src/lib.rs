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
    // An anagram must be the same size as the original word and a word is not an anagram of
    // itself.
    if input.len() == possible.len() && input.to_owned() != possible {
        // We are attempting to preserve characters that use grapheme clusters (aka are complex
        // unicode characters that use multiple bytes).
        // For technical details: https://unicode.org/reports/tr29/
        for c in input.graphemes(true) {
            match possible.find(c) {
                Some(position) => {
                    possible.remove(position);
                }
                None => {
                    return false;
                }
            }
        }
        return true;
    } else {
        false
    }
}
