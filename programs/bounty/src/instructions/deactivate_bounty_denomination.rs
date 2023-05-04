//! Add denomination lets the owner of the protocol to add more
//! supported tokes to denominate
//!

use crate::{
    state::{Denomination, Protocol},
    utils::*,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use std::mem::size_of;

#[derive(Accounts)]
pub struct DeactivateBountyDenomination<'info> {
    // creator
    #[account(mut)]
    pub creator: Signer<'info>,

    /// mint to be used for denomination
    pub mint: Account<'info, Mint>,

    /// bounty denoination to be created
    #[account(
      mut,
      seeds = [
        BOUNTY_SEED.as_bytes(),
        DENOMINATION_SEED.as_bytes(),
        mint.key().to_bytes().as_ref()
        ],
        bump = denomination.bump,
        constraint = denomination.active,
        constraint = denomination.mint.eq(&mint.key()),
    )]
    pub denomination: Box<Account<'info, Denomination>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DeactivateBountyDenomination>) -> Result<()> {
    let denomination = &mut ctx.accounts.denomination;

    // Initialize denomination
    denomination.deactivate()?;

    Ok(())
}
