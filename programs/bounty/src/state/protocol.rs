use anchor_lang::prelude::*;

#[account]
pub struct Protocol {
    pub bump: u8,

    pub bump_seed: [u8; 1],

    pub owner: Pubkey,

    pub fee_collector: Pubkey,

    /// user collection is NFT collection used in the
    /// contract to reward users
    pub user_collection: Pubkey,

    pub sand_token_account: Pubkey,
}

impl Protocol {
    pub fn initialize(
        &mut self,
        bump: &u8,
        fee_collector: &Pubkey,
        owner: &Pubkey,
        user_collection: &Pubkey,
        sand_token_account: &Pubkey,
    ) -> Result<()> {
        self.owner = *owner;
        self.bump = *bump;
        self.bump_seed = [*bump; 1];
        self.user_collection = *user_collection;
        self.fee_collector = *fee_collector;
        self.sand_token_account = *sand_token_account;
        Ok(())
    }

    // update_fee_collector allows to update the fee collector
    pub fn update_fee_collector(&mut self, fee_collector: &Pubkey) -> Result<()> {
        self.fee_collector = *fee_collector;
        Ok(())
    }
}
