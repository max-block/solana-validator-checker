use serde::{Deserialize, Serialize};
use std::{error::Error, process::Command};

use crate::{Config, Validators};

#[derive(Serialize, Deserialize, Debug)]
struct ValidatorsResponse {
    #[serde(rename = "currentValidators")]
    current_validators: Vec<ValidatorItem>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ValidatorItem {
    #[serde(rename = "identityPubkey")]
    identity_pubkey: String,
    #[serde(rename = "voteAccountPubkey")]
    vote_account_pubkey: String,
}

pub fn check_validators(config: &Config, validators: &Validators) -> Result<(), Box<dyn Error>> {
    let shell_command = format!("ssh {} '{} validators --output json'", config.host, config.solana_path);
    let output = Command::new("sh").args(&["-c", &shell_command]).output()?;
    let res = std::str::from_utf8(&output.stdout)?;

    let validators_resp: ValidatorsResponse = serde_json::from_str(res)?;
    println!("\n\ncheck: solana validators");
    for validator in validators.validators.iter() {
        let res = validators_resp
            .current_validators
            .iter()
            .find(|v| v.identity_pubkey == validator.validator);
        println!("{}\t\t\t{}", validator.validator, res.is_some());
    }

    Ok(())
}
