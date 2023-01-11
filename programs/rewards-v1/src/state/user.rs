use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserProfile {
    wallet_address: Pubkey,
    user_profile: String,
    social_media: String,
    mint: Pubkey,
}

impl UserProfile {
    pub fn initialize(
        &self,
        wallet_address: &Pubkey,
        user_profile: &str,
        social_media: &str,
        mint: &Pubkey,
    ) {
        return;
    }
}
