use std::fs;

use crate::error::AocError;

#[derive(Debug)]
pub struct Day1Error {
    pub error_message: String,
}

pub fn day_1(file_path: &String) -> Result<u32, AocError> {
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
