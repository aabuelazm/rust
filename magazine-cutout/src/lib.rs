use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut cutout_words: HashMap<&str, u32> = HashMap::new();

    for word in magazine {
        match *word {
            magazine_word if cutout_words.contains_key(magazine_word) => {
                if let Some(x) = cutout_words.get_mut(magazine_word) {
                    *x = *x + 1;
                }
            }
            magazine_word => {
                cutout_words.insert(magazine_word, 1 as u32);
            }
        }
    }

    for word in note {
        match *word {
            note_word if cutout_words.contains_key(note_word) => {
                if let Some(x) = cutout_words.get_mut(note_word) {
                    if *x >= 1 {
                        *x = *x - 1;
                    } else {
                        return false;
                    }
                }
            }
            _ => {
                return false;
            }
        }
    }

    true
}
