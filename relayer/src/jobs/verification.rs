use std::fs;

use anchor_client::{
    anchor_lang::{prelude::Signer, solana_program::example_mocks::solana_sdk::signature::Keypair},
    solana_sdk::{pubkey::Pubkey, signature::read_keypair_file},
};

pub struct UserLink {
    domain: String,
    user_profile: String,
    wallet: Pubkey,
}

pub enum RelayerError {}

/// get_domains calls the rewards contract and get the
/// potential domains
///
/// FIXME
pub fn get_domains() -> Result<(), RelayerError> {
    let key = read_keypair_file("../../verification.so").expect("Read to file");
    Ok(())
}

/// search_verifications
///
/// Load verification list and only try to insert new entries
pub fn search_verifications() -> Result<Vec<UserLink>, RelayerError> {
    let new_users: Vec<UserLink> = Vec::new();

    Ok(new_users)
}
