use core::num;
use std::{io::Read, ops::Div};

use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use crate::utils::{BlizzardError, BOUNTY_SEED, FEE_REC};

use super::Relayer;

#[account]
#[derive(Debug)]
pub struct Bounty {
    pub bump: u8,
    /// for the seeds
    pub bump_array: [u8; 1],

    pub escrow_bump: u8,
    pub domain: String,
    pub sub_domain: String,
    pub id: String,

    /// Owner of bounty
    pub owner: Pubkey,
    pub mint: Pubkey,

    /// State - created, closed
    pub state: String,

    /// escrow of the bounty
    pub escrow: Pubkey,

    pub bounty_amount: u64,

    pub completed_by: Vec<Pubkey>,
}

impl Bounty {
    /// bounty seeds used to sign transactions
    pub fn seeds(&self) -> [&[u8]; 5] {
        [
            BOUNTY_SEED.as_bytes().as_ref(),
            self.domain.as_bytes(),
            self.sub_domain.as_bytes(),
            self.id.as_bytes(),
            self.bump_array.as_ref(),
        ]
    }

    /// can_complete
    ///
    /// checks if the signer is the owner of the bounty
    ///
    pub fn can_complete(&self, _signer: &Signer, _relayer: &Relayer) {}

    pub fn is_owner(&self, user: &Pubkey) -> bool {
        self.owner.eq(user)
    }

    pub fn create_bounty(
        &mut self,
        bump: &u8,
        owner: &Pubkey,
        escrow: &Pubkey,
        domain: &str,
        sub_domain: &str,
        id: &str,
        bounty_amount: u64,
        mint: &Pubkey,
        escrow_bump: &u8,
    ) -> Result<()> {
        if self.state == "completed" {
            return Err(BlizzardError::CanNotReinitBounty.into());
        }

        self.bump = *bump;
        self.bump_array = [*bump; 1];
        self.domain = domain.to_string();
        self.sub_domain = sub_domain.to_string();
        self.id = id.to_string();
        self.owner = *owner;
        self.state = "started".to_string();
        self.escrow = *escrow;
        self.mint = *mint;
        self.escrow_bump = *escrow_bump;
        self.bounty_amount = bounty_amount;
        Ok(())
    }

    pub fn complete_bounty<'info>(
        &mut self,
        solvers: Vec<&Account<'info, TokenAccount>>,
        fee_collector: &Account<'info, TokenAccount>,
    ) -> Result<Vec<(AccountInfo<'info>, u64)>> {
        self.state = "completed".to_string();
        //self.completed_by = solvers.iter().map(|solver| solver.owner).collect();

        let total_amount = self.bounty_amount;
        let num_solvers = solvers.len();
        let fee = total_amount.div(FEE_REC);
        let amount_per_solver = (total_amount - fee).div(num_solvers as u64);
        let mut bounty_payout = solvers
            .iter()
            .map(|solver| (solver.to_account_info(), amount_per_solver))
            .collect::<Vec<(AccountInfo<'info>, u64)>>();
        bounty_payout.push((fee_collector.to_account_info(), fee));

        Ok(bounty_payout)
    }
}
