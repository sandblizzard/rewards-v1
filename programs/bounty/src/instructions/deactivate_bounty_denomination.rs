//! Add denomination lets the owner of the protocol to add more
//! supported tokes to denominate
//!

use crate::{state::Denomination, utils::*};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

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
        BOUNTY_SEED,
        DENOMINATION_SEED,
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
