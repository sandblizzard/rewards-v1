use anchor_lang::prelude::*;

#[account]
pub struct Protocol {
    owner: Pubkey,

    /// user collection is NFT collection used in the
    /// contract to reward users
    user_collection: Pubkey,
}

impl Protocol {
    pub fn initialize(&mut self, owner: &Pubkey, user_collection: &Pubkey) -> Result<()> {
        self.owner = *owner;
        Ok(())
    }
}
