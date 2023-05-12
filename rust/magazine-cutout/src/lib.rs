// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut word_needed = HashMap::new();
    for word in magazine {
        *word_needed.entry(word).or_default() += 1;
    }
    for word in note {
        *word_needed.entry(word).or_default() -= 1;
        if word_needed.get(word) < (Some(&0)) {
            return false;
        }
    }
    return true;
}
