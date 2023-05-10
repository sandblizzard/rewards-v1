use anchor_lang::prelude::*;

use crate::utils::BOUNTY_SEED;

#[account]
pub struct Protocol {
    pub bump: u8,

    pub bump_seed: [u8; 1],

    pub owner: Pubkey,

    /// user collection is NFT collection used in the
    /// contract to reward users
    pub user_collection: Pubkey,

    pub sand_token_account: Pubkey,
}

impl Protocol {
    pub fn seeds(&self) -> [&[u8]; 2] {
        [BOUNTY_SEED.as_bytes(), &self.bump_seed]
    }

    pub fn account_seed(&self) -> &[u8] {
        BOUNTY_SEED.as_bytes()
    }

    pub fn initialize(
        &mut self,
        bump: &u8,
        owner: &Pubkey,
        user_collection: &Pubkey,
        sand_token_account: &Pubkey,
    ) -> Result<()> {
        self.owner = *owner;
        self.bump = *bump;
        self.bump_seed = [*bump; 1];
        self.user_collection = *user_collection;
        self.sand_token_account = *sand_token_account;
        Ok(())
    }
}
