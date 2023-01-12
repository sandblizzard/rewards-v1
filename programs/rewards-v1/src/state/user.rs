use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserProfile {
    wallet_address: Pubkey,
    user_profile: String,
    social_media: String,
    nft_mint: Pubkey,
}

impl UserProfile {
    pub fn initialize(
        &mut self,
        wallet_address: &Pubkey,
        user_profile: &str,
        social_media: &str,
        nft_mint: &Pubkey,
    ) -> Result<()> {
        self.wallet_address = *wallet_address;
        self.user_profile = user_profile.to_string();
        self.social_media = social_media.to_string();
        self.nft_mint = *nft_mint;
        Ok(())
    }
}
