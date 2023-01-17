use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{
    state::Bounty,
    state::Relayer,
    utils::{BlizzardError, BOUNTY_SEED},
};

#[derive(Accounts)]
pub struct CompleteBounty<'info> {
    /// only owners or relayers can complete bounties
    #[account(mut)]
    pub payer: Signer<'info>,

    /// relayer that wants to complete the transaction
    /// validate the seeds
    #[account(
        seeds=[BOUNTY_SEED.as_bytes(), relayer.owner.key().to_bytes().as_ref()],
        bump = relayer.bump
    )]
    pub relayer: Account<'info, Relayer>,

    /// bounty to be completed
    #[account(mut)]
    pub bounty: Account<'info, Bounty>,

    #[account(
        constraint = escrow.key().eq(&bounty.escrow)
    )]
    pub escrow: Account<'info, TokenAccount>,

    /// up to 4 receivers
    pub receiver1: Account<'info, TokenAccount>,
    pub receiver2: Option<Account<'info, TokenAccount>>,
    pub receiver3: Option<Account<'info, TokenAccount>>,
    pub receiver4: Option<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<CompleteBounty>) -> Result<()> {
    let relayer = &ctx.accounts.relayer;
    let payer = &ctx.accounts.payer;
    let bounty = &mut ctx.accounts.bounty;

    if !(bounty.is_owner(&payer.key()) || relayer.is_owner(&payer.key())) {
        return Err(BlizzardError::NotAuthToCompleteBounty.into());
    } else {
        // create receivers vec
        let mut receivers = Vec::new();
        let receiver1 = &ctx.accounts.receiver1;
        receivers.push(receiver1);
        let r2 = &ctx.accounts.receiver2;
        let r3 = &ctx.accounts.receiver3;
        let r4 = &ctx.accounts.receiver4;

        if r2.is_some() {
            receivers.push(r2.as_ref().unwrap())
        }
        if r3.is_some() {
            receivers.push(r3.as_ref().unwrap())
        }
        if r4.is_some() {
            receivers.push(r4.as_ref().unwrap())
        }
        // complete bounty
        let _bounty_amount = bounty.complete_bounty()?;

        // transfer funds
        Ok(())
    }
}
