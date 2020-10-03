use check::{validator_info::check_validator_info, validators::check_validators};
use serde::{Deserialize, Serialize};
use std::{env, error::Error, fmt::Display, fs};

mod check;
#[derive(Debug)]
pub struct Config {
    solana_path: String,
    host: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Validators {
    validators: Vec<Validator>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Validator {
    validator: String,
    vote: String,
}

impl Validators {
    fn from_file(path: &str) -> Result<Validators, Box<dyn Error>> {
        let validators: Validators = serde_yaml::from_str(&fs::read_to_string(path)?)?;
        Ok(validators)
    }
}

struct AppError {
    pub message: String,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {}", self.message)
    }
}

impl Config {
    pub fn from_env() -> Result<Config, Box<dyn Error>> {
        dotenv::dotenv()?;
        let solana_path = env::var("SOLANA_PATH")?;
        let host = env::var("HOST")?;
        Ok(Config { solana_path, host })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let validators = Validators::from_file("tmp/validators.yml")?;

    check_validators(&config, &validators)?;
    check_validator_info(&config, &validators)?;
    Ok(())
}
