use anchor_lang::{prelude::*, Bump};

use crate::utils::{BlizzardError, BOUNTY_SEED};

use super::Relayer;

#[account]
pub struct Bounty {
    bump: u8,
    /// for the seeds
    bump_array: [u8; 1],
    domain: String,
    sub_domain: String,
    area: String,
    id: String,

    /// Owner of bounty
    owner: Pubkey,

    /// State - created, closed
    state: String,

    /// escrow of the bounty
    pub escrow: Pubkey,

    bounty_amount: u64,
}

impl Bounty {
    /// bounty seeds used to sign transactions
    pub fn seeds(&self) -> [&[u8]; 6] {
        [
            BOUNTY_SEED.as_bytes().as_ref(),
            self.domain.as_bytes(),
            self.sub_domain.as_bytes(),
            self.area.as_bytes(),
            self.id.as_bytes(),
            self.bump_array.as_ref(),
        ]
    }

    /// can_complete
    ///
    /// checks if the signer is the owner of the bounty
    ///
    pub fn can_complete(&self, signer: &Signer, relayer: &Relayer) {}

    pub fn is_owner(&self, user: &Pubkey) -> bool {
        self.owner.eq(user)
    }

    pub fn create_bounty(
        &self,
        bump: &u8,
        owner: &Pubkey,
        escrow: &Pubkey,
        domain: &str,
        sub_domain: &str,
        area: &str,
        id: &str,
        bounty_amount: u64,
    ) -> Result<Bounty> {
        if (self.state == "completed") {
            return Err(BlizzardError::CanNotReinitBounty.into());
        }

        Ok(Bounty {
            bump: *bump,
            bump_array: [*bump; 1],
            domain: domain.to_string(),
            sub_domain: sub_domain.to_string(),
            area: area.to_string(),
            id: id.to_string(),
            owner: owner.clone(),
            state: "started".to_string(),
            escrow: escrow.clone(),
            bounty_amount,
        })
    }

    pub fn complete_bounty(&mut self) -> Result<u64> {
        self.state = "completed".to_string();
        Ok(self.bounty_amount)
    }
}
