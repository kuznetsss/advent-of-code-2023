use crate::error::AocError;

use reqwest::blocking;
use serde::Deserialize;
use serde_yaml;

use std::fs;
use std::path;

macro_rules! URL_TEMPLATE {
    () => {
        "https://adventofcode.com/2023/day/{}/input"
    };
}

const CACHE_PATH: &str = "./.cache/input";
const CONFIG_FILE: &str = "./advent_of_code.yml";

#[derive(Debug, Deserialize)]
struct Config {
    cookie_session: String,
}

pub struct InputProvider {
    config: Config,
}

impl InputProvider {
    pub fn new() -> Result<Self, AocError> {
        let config_str = fs::read_to_string(&CONFIG_FILE)
            .map_err(|e| AocError::IoError(e, CONFIG_FILE.to_string()))?;
        let config: Config = serde_yaml::from_str(config_str.as_str())
            .map_err(|e| AocError::CantParseConfig(e.to_string(), CONFIG_FILE.to_string()))?;
        Ok(InputProvider { config })
    }

    pub fn get_input(&self, day_number: u32) -> Result<String, AocError> {
        let cache_path = path::Path::new(CACHE_PATH);
        if !cache_path.exists() {
            fs::create_dir_all(cache_path).unwrap();
        }
        let file_path = cache_path.join(format!("{}.txt", day_number));
        if file_path.exists() {
            let content = fs::read_to_string(&file_path)
                .map_err(|e| AocError::IoError(e, file_path.to_string_lossy().to_string()))?;
            return Ok(content);
        }
        let url = format!(URL_TEMPLATE!(), day_number);
        let response = blocking::Client::new()
            .get(url)
            .header("Cookie", format!("session={}", &self.config.cookie_session))
            .send()
            .map_err(|e| AocError::ErrorDownloadingInput(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AocError::ErrorDownloadingInput(
                response.status().to_string(),
            ));
        }
        println!("Downloaded input for day {day_number}");
        let input_text = response.text().unwrap();
        fs::write(&file_path, &input_text)
            .map_err(|e| AocError::IoError(e, file_path.to_string_lossy().to_string()))?;
        Ok(input_text)
    }
}
