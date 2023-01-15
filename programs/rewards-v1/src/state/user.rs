use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use std::convert::TryFrom;

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

    pub fn try_get_data(&self) -> Result<Vec<u8>> {
        let serialized = vec![];
        BorshSerialize::try_to_vec(self)?;
        Ok(serialized)
    }
    
    pub fn get_user_profile(&self) -> &str {
        &self.user_profile
    }

    pub fn get_user_socials(&self) -> &str {
        &self.social_media
    }

//     #[test]
// fn test_try_get_data() {
//     let mut user_profile = UserProfile::default();
//     let wallet_address = Pubkey::new_rand();
//     let user_profile = "Axelofwar";
//     let social_media = "axelofwar_twitter";
//     let nft_mint = Pubkey::new_rand();
//     user_profile.initialize(&wallet_address, &user_profile, &social_media, &nft_mint).unwrap();
//     let data = user_profile.try_get_data();
//     assert!(data.is_ok());
// }
}
