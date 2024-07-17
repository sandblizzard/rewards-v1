use std::ops::Div;

use anchor_lang::prelude::*;
use borsh::BorshDeserialize;

use crate::{
    utils::{BlizzardError, BOUNTY_SEED, FEE_REC},
    Solver, TSolver,
};

use super::Relayer;

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq)]
pub enum BountyState {
    Created = 0,
    Completed = 1,
}

#[account]
pub struct Bounty {
    /// Owner of bounty
    pub owner: Pubkey,
    pub mint: Pubkey,
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

    /// WHo completed the bounty
    pub completed_by: Option<Pubkey>,

    pub created_at: i64,
    pub updated_at: i64,
    pub ends_at: Option<i64>,

    // external id of the bounty. Could be bounty ID
    // In the case of github issues this would be the issue id
    pub external_id: String,

    // title of the bounty
    pub title: String,

    // description of the bounty
    pub description: String,

    // pubkeys that contributed to the bounty
    pub donaters: Vec<Pubkey>,
    pub donate_amount: Vec<u64>,

    // pubkeys that solved the bounty
    pub solvers: Vec<Pubkey>,
    pub solver_solutions: Vec<Pubkey>,

    pub solved_by: Vec<Pubkey>,
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
        title: &str,
        description: &str,
        id: &u64,
        external_id: &str,
        owner: &Pubkey,
        escrow: &Pubkey,
        domain: &Pubkey,
        mint: &Pubkey,
        escrow_bump: &u8,
        ends_at: Option<i64>,
    ) -> Result<()> {
        if self.state == BountyState::Completed {
            return Err(BlizzardError::CanNotReinitBounty.into());
        }
        self.owner = *owner;
        self.mint = *mint;

        self.bump = *bump;
        self.bump_array = [*bump; 1];

        self.escrow = *escrow;
        self.escrow_bump = *escrow_bump;

        // init DomainIdentificator
        self.domain = *domain;

        self.id_bytes = id.to_le_bytes();
        self.external_id = external_id.to_string();

        self.created_at = Clock::get()?.unix_timestamp;
        self.updated_at = Clock::get()?.unix_timestamp;
        // ends in three days
        self.ends_at = ends_at;
        self.state = BountyState::Created;
        self.escrow = *escrow;

        self.escrow_bump = *escrow_bump;

        self.title = title.to_string();
        self.description = description.to_string();

        self.donaters = vec![];
        self.donate_amount = vec![];

        self.solvers = vec![];
        self.solver_solutions = vec![];

        self.solved_by = vec![];
        Ok(())
    }

    /// donate_to_bounty
    pub fn donate_to_bounty(&mut self, donater: &Pubkey, amount: u64) -> Result<()> {
        if self.state == BountyState::Completed {
            return Err(BlizzardError::BountyIsCompleted.into());
        }
        // if donater is in list, just uprate amount
        if let Some(index) = self.donaters.iter().position(|x| x == donater) {
            self.donate_amount[index] += amount;
            return Ok(());
        }

        self.donaters.push(*donater);
        self.donate_amount.push(amount);
        Ok(())
    }

    /// Propose solution to bounty
    pub fn propose_solution(&mut self, solver: &Pubkey, solution: &Pubkey) -> Result<()> {
        if self.state == BountyState::Completed {
            return Err(BlizzardError::BountyIsCompleted.into());
        }
        self.solvers.push(*solver);
        self.solver_solutions.push(*solution);
        Ok(())
    }

    pub fn calculate_mining_rewards<'info>(
        &'info mut self,
        solvers: &'info Vec<Box<dyn TSolver>>,
        fee_collector: &'info Box<dyn TSolver>,
        completer: &Pubkey,
    ) -> Result<Vec<(&Box<dyn TSolver>, u64)>> {
        self.state = BountyState::Completed;

        let total_amount = self.donate_amount.iter().sum::<u64>();
        let num_solvers = solvers.len();
        let fee = total_amount.div(FEE_REC);
        let amount_per_solver = (total_amount - fee).div(num_solvers as u64);

        let mut bounty_payout: Vec<(&Box<dyn TSolver>, u64)> = solvers
            .iter()
            .map(|solver| (solver, amount_per_solver))
            .collect();
        bounty_payout.push((&fee_collector, amount_per_solver));

        // update state
        self.solved_by = vec![*completer];
        self.state = BountyState::Completed;

        Ok(bounty_payout)
    }

    pub fn complete_bounty<'info>(
        &mut self,
        completer: Pubkey,
        solvers: Vec<Box<dyn TSolver>>,
    ) -> Result<()> {
        self.completed_by = Some(completer);
        self.solved_by = solvers.iter().map(|s| s.get_owner()).collect();
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
            created_at: 0,
            updated_at: 0,
            ends_at: Some(0),
            external_id: "0".to_string(),
            title: "title".to_string(),
            description: "description".to_string(),
            donaters: vec![],
            donate_amount: vec![],
            solvers: vec![],
            solver_solutions: vec![],
            completed_by: None,
            solved_by: vec![],
        };
        assert!(bounty.is_owner(&owner));
    }

    #[test]
    fn test_complete_bounty() {
        // TODO: try to close bounty
    }
}
