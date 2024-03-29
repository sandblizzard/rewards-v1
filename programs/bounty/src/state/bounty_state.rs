use std::ops::Div;

use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use crate::utils::{BlizzardError, BOUNTY_SEED, FEE_REC};

use super::Relayer;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub enum BountyState {
    Created = 0,
    Completed = 1,
}

#[account]
#[derive(Debug)]
pub struct Bounty {
    pub id: String,

    pub bump: u8,
    /// for the seeds
    pub bump_array: [u8; 1],
    pub escrow_bump: u8,

    /// Owner of bounty
    pub owner: Pubkey,
    pub mint: Pubkey,

    /// State - created, closed
    pub state: BountyState,

    /// escrow of the bounty
    pub escrow: Pubkey,

    /// domain information
    pub domain: Pubkey,
    /// domain as bytes
    pub domain_bytes: Vec<u8>,

    pub bounty_amount: u64,

    pub completed_by: Option<Pubkey>,
}

impl Bounty {
    /// bounty seeds used to sign transactions
    pub fn signing_seeds(&self) -> [&[u8]; 3] {
        [
            BOUNTY_SEED.as_bytes(),
            self.id.as_bytes(),
            self.bump_array.as_ref(),
        ]
    }

    pub fn seeds(&self) -> [&[u8]; 2] {
        [BOUNTY_SEED.as_bytes(), self.id.as_bytes()]
    }

    /// can_complete
    ///
    /// checks if the signer is the owner of the bounty
    ///
    pub fn can_complete(&self, _signer: &Signer, _relayer: &Relayer) {}

    pub fn is_owner(&self, user: &Pubkey) -> bool {
        self.owner.eq(user)
    }

    #[warn(clippy::too_many_arguments)]
    pub fn create_bounty(
        &mut self,
        bump: &u8,
        id: &str,
        owner: &Pubkey,
        escrow: &Pubkey,
        domain: &Pubkey,
        bounty_amount: u64,
        mint: &Pubkey,
        escrow_bump: &u8,
    ) -> Result<()> {
        if self.state == BountyState::Completed {
            return Err(BlizzardError::CanNotReinitBounty.into());
        }

        // init DomainIdentificator
        self.domain = *domain;
        self.domain_bytes = domain.to_bytes().to_vec();

        // set remaining fields
        self.bump = *bump;
        self.bump_array = [*bump; 1];
        self.owner = *owner;
        self.state = BountyState::Created;
        self.escrow = *escrow;
        self.mint = *mint;
        self.id = id.to_string();
        self.escrow_bump = *escrow_bump;
        self.bounty_amount = bounty_amount;
        self.completed_by = None;
        Ok(())
    }

    pub fn complete_bounty<'info>(
        &mut self,
        solvers: Vec<&Account<'info, TokenAccount>>,
        fee_collector: &Account<'info, TokenAccount>,
        completer: &Pubkey,
    ) -> Result<Vec<(AccountInfo<'info>, u64)>> {
        self.state = BountyState::Completed;

        let total_amount = self.bounty_amount;
        let num_solvers = solvers.len();
        let fee = total_amount.div(FEE_REC);
        let amount_per_solver = (total_amount - fee).div(num_solvers as u64);
        let mut bounty_payout = solvers
            .iter()
            .map(|solver| (solver.to_account_info(), amount_per_solver))
            .collect::<Vec<(AccountInfo<'info>, u64)>>();
        bounty_payout.push((fee_collector.to_account_info(), fee));

        // update state
        self.completed_by = Some(*completer);
        self.state = BountyState::Completed;

        Ok(bounty_payout)
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;
    #[test]
    fn test_creation() {
        let owner = Pubkey::from_str("Acyq4k7tJ38DyG4kppEEUF9AH1Cuiw7cGCfBuoEh8zH9").unwrap();
        let bounty = Bounty {
            bump: 0,
            bump_array: [0; 1],
            escrow_bump: 0,
            domain: Pubkey::new_unique(),
            domain_bytes: [0; 32].to_vec(),
            id: "".to_string(),
            owner,
            mint: Pubkey::new_unique(),
            state: BountyState::Created,
            escrow: Pubkey::new_unique(),
            bounty_amount: 0,
            completed_by: None,
        };
        assert_eq!(bounty.is_owner(&owner), true);
    }

    #[test]
    fn test_complete_bounty() {
        // TODO: try to close bounty
    }
}
