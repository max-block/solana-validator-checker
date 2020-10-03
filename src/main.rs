use std::error::Error;

use solana_validator_checker::{run, Config};

fn main() -> Result<(), Box<dyn Error>> {
    run(Config::from_env()?)?;

    Ok(())
}
