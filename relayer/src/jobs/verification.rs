use crate::{
    domains::{github::get_octocrab_instance, utils::SBError},
    external::{get_sandblizzard_collection, UnderdogCollection},
};
use anchor_client::solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};
use base64::engine::general_purpose;
use serde::{Deserialize, Serialize};
use std::env;

use std::{fs::File, result::Result};
pub struct UserLink {
    domain: String,
    user_profile: String,
    wallet: Pubkey,
}

/// get_domains calls the rewards contract and get the
/// potential domains
///
/// FIXME
pub fn get_domains() -> Result<(), SBError> {
    let _key = read_keypair_file("../../verification.so").expect("Read to file");
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    github: String,
    #[serde(rename = "solanaAddress")]
    solana_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct VerificationData {
    name: String,
    profiles: Vec<Profile>,
}

/// verify_users
///
/// reads the PRs into the verification list and
/// mints NFTs if they don't exist already
pub async fn verify_users() -> Result<(), SBError> {
    let gh = get_octocrab_instance()?;

    let mut verification = gh
        .repos("sandblizzard", "verification")
        .get_content()
        .path("profile.list.json")
        .r#ref("main")
        .send()
        .await
        .map_err(|err| {
            SBError::FailedToGetVerficationFile("verify_users".to_string(), err.to_string())
        })?;

    let files = verification.take_items();
    if files.len() != 1 {
        return Err(SBError::UnexpectedNumberOfElements(
            "verify_users".to_string(),
            1,
            files.len() as u16,
        ));
    }

    let decoded_file = files[0].decoded_content().unwrap();

    let parsed_file: VerificationData = serde_json::from_str(&decoded_file)
        .map_err(|err| SBError::FailedToParseFile("verify_users".to_string(), err.to_string()))?;
    log::debug!("[verify_users] parsed_file {:?}", parsed_file);
    // try generate the users using underdog
    let profiles = parsed_file.profiles;
    let blizzard_collection = get_sandblizzard_collection()?;
    let collection = UnderdogCollection::new(&blizzard_collection.to_string());
    for profile in profiles {
        // create user
        collection
            .mint_nft(&profile.solana_address, &profile.github)
            .await?;
    }
    Ok(())
}
