use std::{cmp::Ordering, collections::HashMap};

use crate::error::AocError;

pub fn day_3(input: &String) -> Result<u32, AocError> {
    let index = SymbolsIndex::new(input);
    let mut sum = 0;
    for (line_ind, line) in input.lines().enumerate() {
        let mut it = line.chars().enumerate().peekable();
        while let Some((start, c)) = it.next() {
            if !c.is_numeric() {
                continue;
            }

            let end = it.find(|(_, c)| !c.is_numeric()).unwrap_or((line.chars().count(), '.')).0;

            if index.has_symbol_around(line_ind, start, end) {
                let number: u32 = line[start..end].parse().unwrap();
                sum += number;
            }
        }
    }
    Ok(sum)
}

#[derive(Debug)]
struct SymbolsIndex {
    chars_map: HashMap<usize, Vec<usize>>,
}

impl SymbolsIndex {
    fn new(input: &String) -> Self {
        let mut index = SymbolsIndex {
            chars_map: HashMap::new(),
        };
        for (line_ind, line) in input.lines().enumerate() {
            line.chars()
                .enumerate()
                .filter(|(_, c)| !c.is_numeric() && c != &'.')
                .for_each(|(i, _)| {
                    index
                        .chars_map
                        .entry(line_ind)
                        .or_insert(Vec::<usize>::new())
                        .push(i)
                });
        }
        index
    }

    fn has_symbol_around(&self, line_ind: usize, start: usize, end: usize) -> bool {
        let start = if start == 0 { 0 } else { start - 1 };

        // Check the line above
        if line_ind != 0 {
            // print!(" Checking previous line {}:{} - {}", line_ind- 1, start, end);
            if let Some(v) = self.chars_map.get(&(line_ind - 1)) {
                if let Some(ind) = lower_bound(v, start) {
                    if ind <= end {
                        return true;
                    }
                }
            }
        }

        // Check current line
        if let Some(v) = self.chars_map.get(&line_ind) {
            // print!(" Checking current line {}:{} - {}", line_ind- 1, start, end);
            if start > 0 && v.binary_search(&start).is_ok() {
                return true;
            }
            if v.binary_search(&end).is_ok() {
                return true;
            }
        }

        // Check the next line
        if let Some(v) = self.chars_map.get(&(line_ind + 1)) {
            if let Some(ind) = lower_bound(v, start) {
                if ind <= end {
                    return true;
                }
            }
        }

        false
    }
}

fn lower_bound(v: &Vec<usize>, n: usize) -> Option<usize> {
    let upper_bound = v
        .binary_search_by(|i| match i.cmp(&n) {
            Ordering::Equal => Ordering::Greater,
            ord => ord,
        })
        .unwrap_or_else(|i| i);
    if upper_bound < v.len() {
        Some(v.get(upper_bound).unwrap().to_owned())
    } else {
        None
    }
}
