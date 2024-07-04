use anchor_lang::prelude::*;

// Solver trait that needs to be implemented by the protocol
pub trait TSolver {
    fn claim_rewards(&mut self) -> Result<()>;
    fn update_rewards(&mut self, rewards: u64) -> Result<()>;
    fn get_owner(&self) -> Pubkey;
}

#[account]
pub struct ProtocolCollector {
    pub bump: u8,

    pub bump_seed: [u8; 1],

    // owner of the protocol
    // it has the ability to update it
    pub owner: Pubkey,

    pub mint: Pubkey,

    pub sand_mint: Pubkey,

    pub claimable_sand: u64,
    pub claimable_fee: u64,
}

pub enum AnySolver {
    ProtocolCollector,
    Solver,
}

impl TSolver for ProtocolCollector {
    fn claim_rewards(&mut self) -> Result<()> {
        self.claimable_sand = 0;
        self.claimable_fee = 0;
        Ok(())
    }

    fn update_rewards(&mut self, rewards: u64) -> Result<()> {
        self.claimable_sand += rewards;
        Ok(())
    }

    fn get_owner(&self) -> Pubkey {
        self.owner
    }
}
