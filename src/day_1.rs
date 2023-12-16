use crate::error::AocError;

pub fn day_1(input: &String) -> Result<u32, AocError> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        sum += sum_in_line(line)?;
    }
    Ok(sum)
}

fn sum_in_line(line: &str) -> Result<u32, AocError> {
    let mut digits = Vec::new();
    for (i, c) in line.chars().enumerate() {
        if c.is_numeric() {
            digits.push(c.to_digit(10).unwrap());
            continue;
        }
        for (s, n) in DIGITS {
            if line[i..].starts_with(s) {
                digits.push(n);
                break;
            }
        }
    }
    if digits.is_empty() {
        return Err(AocError::DigitNotFound(line.to_string()));
    }
    Ok(digits.first().unwrap() * 10 + digits.last().unwrap())
}

const DIGITS: [(&'static str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];
