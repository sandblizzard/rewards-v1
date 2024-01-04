use anchor_lang::prelude::*;

use crate::utils::BOUNTY_SEED;

#[account]
pub struct Protocol {
    pub bump: u8,

    pub bump_seed: [u8; 1],

    // owner of the protocol
    // it has the ability to update it
    pub owner: Pubkey,

    pub sand_mint: Pubkey,
}

impl Protocol {
    pub fn seeds(&self) -> [&[u8]; 2] {
        [BOUNTY_SEED.as_bytes(), &self.bump_seed]
    }

    pub fn account_seed(&self) -> &[u8] {
        BOUNTY_SEED.as_bytes()
    }

    pub fn initialize(&mut self, bump: &u8, owner: &Pubkey, mint: &Pubkey) -> Result<()> {
        self.owner = *owner;
        self.bump = *bump;
        self.bump_seed = [*bump; 1];
        self.sand_mint = *mint;
        Ok(())
    }
}
