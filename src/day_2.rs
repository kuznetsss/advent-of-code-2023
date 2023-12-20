use crate::error::AocError;

pub fn day_2(input: &String) -> Result<u32, AocError> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let game = Game::new(line)?;
        if game.red <= RED_MAX && game.blue <= BLUE_MAX && game.green <= GREEN_MAX {
            sum += game.id;
        }
    }
    Ok(sum)
}

pub fn day_2_part_2(input: &String) -> Result<u32, AocError> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let game = Game::new(line)?;
        sum += game.green * game.blue * game.red;
    }
    Ok(sum)
}

const RED_MAX: u32 = 12;
const GREEN_MAX: u32 = 13;
const BLUE_MAX: u32 = 14;

struct Game {
    id: u32,
    red: u32,
    blue: u32,
    green: u32,
}

impl Game {
    fn new(s: &str) -> Result<Self, AocError> {
        let s = s.strip_prefix("Game ").ok_or(AocError::ParseError(
            "No prefix 'Game '".to_string(),
            s.to_string(),
        ))?;
        let id = s.split(':').nth(0).ok_or(AocError::ParseError(
            "No ':' found".to_string(),
            s.to_string(),
        ))?;
        let id: u32 = id
            .parse::<u32>()
            .map_err(|e| AocError::ParseError(e.to_string(), id.to_string()))?;
        let mut game = Game {
            id,
            red: 0,
            blue: 0,
            green: 0,
        };
        for round in s.split(':').nth(1).unwrap().split(';') {
            for color_and_value in round.trim().split(',') {
                let mut splited = color_and_value.trim().split(' ');
                let value = splited.nth(0).ok_or(AocError::ParseError(
                    "No space in color".to_string(),
                    color_and_value.to_string(),
                ))?;
                let value: u32 = value.parse().unwrap();
                let color = splited.nth(0).ok_or(AocError::ParseError(
                    "No color value".to_string(),
                    color_and_value.to_string(),
                ))?;
                match color {
                    "red" => game.red = std::cmp::max(game.red, value),
                    "blue" => game.blue = std::cmp::max(game.blue, value),
                    "green" => game.green = std::cmp::max(game.green, value),
                    _ => {
                        return Err(AocError::ParseError(
                            "Unexpected color".to_string(),
                            color.to_string(),
                        ))
                    }
                }
            }
        }
        Ok(game)
    }
}
