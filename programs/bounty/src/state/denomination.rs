//!  Denominations allow users to denomintate the bounty
//! in various mints (tokens)
//!

use crate::utils::{BOUNTY_SEED, DENOMINATION_SEED};
use anchor_lang::prelude::*;

#[account]
pub struct Denomination {
    pub bump: u8,

    pub bump_array: [u8; 1],

    pub mint: Pubkey,

    pub active: bool,

    pub fee_collector: Pubkey,
}

impl Denomination {
    pub fn seeds(&self) -> [&[u8]; 3] {
        [
            BOUNTY_SEED,
            DENOMINATION_SEED.as_bytes(),
            //&self.mint_bytes,
            self.bump_array.as_ref(),
        ]
    }
    /// initialize the denoination for a given mint and fee_collector (tokenAccount)
    pub fn initialize(&mut self, bump: &u8, mint: &Pubkey, fee_collector: &Pubkey) -> Result<()> {
        self.bump = *bump;
        self.bump_array = [*bump; 1];
        self.mint = *mint;
        //self.mint_bytes = Box::new(mint.to_bytes());
        self.active = true;
        self.fee_collector = *fee_collector;
        Ok(())
    }

    /// deactivate the denomination
    pub fn deactivate(&mut self) -> Result<()> {
        self.active = false;
        Ok(())
    }
}
