use std::ops::Div;

use anchor_lang::prelude::*;

use crate::{
    utils::{BlizzardError, BOUNTY_SEED, FEE_REC},
    TSolver,
};

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
    /// Owner of bounty
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub bounty_amount: u64,

    /// State - created, closed
    pub state: BountyState,

    pub bump: u8,
    /// for the seeds
    pub bump_array: [u8; 1],
    pub escrow_bump: u8,

    /// escrow of the bounty
    pub escrow: Pubkey,

    /// domain information
    pub domain: Pubkey,

    pub id_bytes: [u8; 8],
    pub completed_by: Option<Pubkey>,
}

impl Bounty {
    /// bounty seeds used to sign transactions
    pub fn signing_seeds(&self) -> [&[u8]; 3] {
        [BOUNTY_SEED, &self.id_bytes, self.bump_array.as_ref()]
    }

    pub fn seeds(&self) -> [&[u8]; 2] {
        [BOUNTY_SEED, &self.id_bytes]
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
        id: &u64,
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

        // set remaining fields
        self.bump = *bump;
        self.bump_array = [*bump; 1];
        self.owner = *owner;
        self.state = BountyState::Created;
        self.escrow = *escrow;
        self.mint = *mint;
        self.id_bytes = id.to_le_bytes();
        self.escrow_bump = *escrow_bump;
        self.bounty_amount = bounty_amount;
        self.completed_by = None;
        Ok(())
    }

    pub fn calculate_mining_rewards<'info>(
        &'info mut self,
        solvers: &'info Vec<Box<dyn TSolver>>,
        fee_collector: &'info Box<dyn TSolver>,
        completer: &Pubkey,
    ) -> Result<Vec<(&Box<dyn TSolver>, u64)>> {
        self.state = BountyState::Completed;

        let total_amount = self.bounty_amount;
        let num_solvers = solvers.len();
        let fee = total_amount.div(FEE_REC);
        let amount_per_solver = (total_amount - fee).div(num_solvers as u64);

        let mut bounty_payout: Vec<(&Box<dyn TSolver>, u64)> = solvers
            .iter()
            .map(|solver| (solver, amount_per_solver))
            .collect();
        bounty_payout.push((&fee_collector, amount_per_solver));

        // update state
        self.completed_by = Some(*completer);
        self.state = BountyState::Completed;

        Ok(bounty_payout)
    }

    pub fn complete_bounty<'info>(&mut self, completer: Pubkey) -> Result<()> {
        self.completed_by = Some(completer);
        self.state = BountyState::Completed;
        Ok(())
    }
}
pub fn calculate_bounty_payout<'a>(
    total_amount: &u64,
    solvers: &Vec<AccountInfo<'a>>,
    fee_collector: &AccountInfo<'a>,
) -> Result<Vec<(AccountInfo<'a>, u64)>> {
    let num_solvers = solvers.len();
    let fee = total_amount.div(FEE_REC);
    let amount_per_solver = (total_amount - fee).div(num_solvers as u64);
    let mut bounty_payout = solvers
        .iter()
        .map(|solver| (solver.clone(), amount_per_solver))
        .collect::<Vec<(AccountInfo, u64)>>();
    bounty_payout.push((fee_collector.clone(), fee));

    Ok(bounty_payout)
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
            id_bytes: [0; 8],
            owner,
            mint: Pubkey::new_unique(),
            state: BountyState::Created,
            escrow: Pubkey::new_unique(),
            bounty_amount: 0,
            completed_by: None,
        };
        assert!(bounty.is_owner(&owner));
    }

    #[test]
    fn test_complete_bounty() {
        // TODO: try to close bounty
    }
}
