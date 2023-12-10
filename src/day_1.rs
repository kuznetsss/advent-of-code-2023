use std::{fs, collections::HashMap};

use crate::error::AocError;


pub fn day_1(file_path: &String) -> Result<u32, AocError> {
    let trie = trie!(vec!("one", "two"));
    let mut sum: u32 = 0;
    let content = fs::read_to_string(file_path).map_err(|e| AocError::IoError(e, file_path.clone()))?;
    for line in content.lines() {
        let mut first_number: u32 = 0;
        let mut last_number: u32 = 0;
        let convert = |c: char| -> u32 {
            c.to_digit(10).unwrap()
        };
        for c in line.chars() {
            if c.is_numeric() {
                first_number = convert(c);
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_numeric() {
                last_number = convert(c);
                break;
            }
        }
        sum += first_number * 10 + last_number;
    }
    Ok(sum)
}

#[derive(Debug)]
struct Trie {
    is_end_of_word : bool,
    next_layer: HashMap<char, Box<Trie>>,
}

impl Trie {
    fn new() -> Self {
        Trie {is_end_of_word: false, next_layer: HashMap::new() }
    }

    fn add_word(&mut self, word: &str) {
        let mut leaf = self;
        for c in word.chars() {
            leaf = leaf.next_layer.entry(c).or_insert(Box::new(Trie::new())).as_mut();
        }
        leaf.is_end_of_word = true;
    }

    /// returns next_layer or None
    fn search(&self, c: &char) -> Option<&Trie> {
        match self.next_layer.get(&c) {
            Some(boxed_trie) => Some(boxed_trie.as_ref()),
            None => None
        }
    }
}

#[macro_use]
macro_rules! trie {
    (v : &expr) => {
        let mut trie = Trie::new();
        for w in v {
            trie.add_word(&w)
        }
        trie
    };
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn search_in_trie() {
        let mut trie = Trie::new();
        let test_word = "trie";
        trie.add_word(&test_word);
        assert!(!trie.is_end_of_word);
        let mut layer = &trie;
        for c in test_word.chars() {
            assert!(layer.search(&'a').is_none());
            let search_result = layer.search(&c);
            assert!(search_result.is_some());
            layer = search_result.unwrap();
            assert_eq!(layer.is_end_of_word, c == 'e');
        }
    }
}
