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
}

impl Protocol {
    pub fn initialize(
        &mut self,
        bump: &u8,
        fee_collector: &Pubkey,
        owner: &Pubkey,
        user_collection: &Pubkey,
    ) -> Result<()> {
        self.owner = *owner;
        self.bump = *bump;
        self.bump_seed = [*bump; 1];
        self.user_collection = *user_collection;
        self.fee_collector = *fee_collector;
        Ok(())
    }
}
