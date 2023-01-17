use crate::domains::{github::get_connection, utils::SBError};
use anchor_client::{
    anchor_lang::{prelude::Signer, solana_program::example_mocks::solana_sdk::signature::Keypair},
    solana_sdk::{pubkey::Pubkey, signature::read_keypair_file},
};
use octocrab;
use serde::{Deserialize, Serialize};
use serde_json::*;
use std::{
    fs::{self, File},
    result::Result,
};
pub struct UserLink {
    domain: String,
    user_profile: String,
    wallet: Pubkey,
}
use reqwest;

/// get_domains calls the rewards contract and get the
/// potential domains
///
/// FIXME
pub fn get_domains() -> Result<(), SBError> {
    let key = read_keypair_file("../../verification.so").expect("Read to file");
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct Profile {
    github: String,
    solanaAddress: String,
}

#[derive(Serialize, Deserialize)]
struct VerificationData {
    name: String,
    profiles: Vec<Profile>,
}

/// get_verification
///
/// reads the PRs into the verification list and
/// mints NFTs if they don't exist already
pub async fn get_verification() -> Result<(), SBError> {
    let gh = get_connection().await?;

    let verification = gh
        .repos("sandblizzard", "verifcation")
        .get_content()
        .path("./data/")
        .r#ref("main")
        .send()
        .await
        .unwrap();

    // load the verification file
    let verification_file = File::open("./data/profile.list.json").unwrap();
    let dd: VerificationData = serde_json::from_reader(verification_file).unwrap();

    // try generate the users using underdog
    let profiles = dd.profiles;
    for profile in profiles {
        // create user
    }
    Ok(())
}

/// search_verifications
///
/// Load verification list and only try to insert new entries
pub fn search_verifications() -> Result<Vec<UserLink>, SBError> {
    let new_users: Vec<UserLink> = Vec::new();

    Ok(new_users)
}
