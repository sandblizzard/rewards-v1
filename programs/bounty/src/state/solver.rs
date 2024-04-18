use anchor_lang::prelude::*;

use crate::TSolver;

#[account]
pub struct Solver {
    pub bump: u8,
    pub bump_seed: [u8; 1],
    pub owner: Pubkey,

    pub mining_mint: Pubkey,

    // rewards
    pub claimable_rewards: u64,

    // total rewards
    pub total_rewards: u64,

    // total solved bounties
    pub total_solved_bounties: u64,

    pub active: bool,
}

impl TSolver for Solver {
    fn claim_rewards(&mut self) -> Result<()> {
        self.claimable_rewards = 0;
        Ok(())
    }

    fn update_rewards(&mut self, rewards: u64) -> Result<()> {
        self.claimable_rewards += rewards;
        self.total_rewards += rewards;
        self.total_solved_bounties += 1;
        Ok(())
    }
}

impl Solver {
    pub fn initialize(&mut self, owner: &Pubkey, bump: &u8, mint: Pubkey) -> Result<()> {
        self.owner = *owner;
        self.bump_seed = [*bump; 1];
        self.bump = *bump;
        self.active = true;
        self.claimable_rewards = 0;
        self.total_rewards = 0;
        self.total_solved_bounties = 0;
        self.mining_mint = mint;
        Ok(())
    }

    pub fn deactivate(&mut self) -> Result<()> {
        self.active = false;
        Ok(())
    }

    pub fn is_owner(&self, user: &Pubkey) -> bool {
        self.owner.eq(user)
    }
}
