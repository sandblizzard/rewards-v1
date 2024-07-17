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
pub struct AddBountyDenomination<'info> {
    // creator
    #[account(mut)]
    pub creator: Signer<'info>,

    /// protocol config
    #[account(
        seeds = [
            BOUNTY_SEED,
        ],
        bump = protocol.bump
    )]
    pub protocol: Account<'info, Protocol>,

    /// mint to be used for denomination
    pub mint: Account<'info, Mint>,

    /// bounty denoination to be created
    #[account(
        init,
        payer = creator,
        seeds = [
            BOUNTY_SEED,
            DENOMINATION_SEED,
            mint.key().to_bytes().as_ref()
        ],
        bump,
        space = 8 + size_of::<Denomination>()
    )]
    pub denomination: Box<Account<'info, Denomination>>,

    /// Fee collector is owned by the protocol and
    /// collects fees from the bounty
    #[account(
        init,
        payer = creator,
        seeds = [
            BOUNTY_SEED,
            FEE_COLLECTOR_SEED.as_bytes(),
            mint.key().to_bytes().as_ref()
        ],
        bump,
        token::mint = mint,
        token::authority = protocol,
    )]
    pub fee_collector: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AddBountyDenomination>) -> Result<()> {
    let denomination = &mut ctx.accounts.denomination;
    let mint = &mut ctx.accounts.mint;
    let fee_collector = &mut ctx.accounts.fee_collector;
    let denomination_bump = &ctx.bumps.denomination;
    // Initialize denomination
    denomination.initialize(denomination_bump, &mint.key(), &fee_collector.key())?;

    Ok(())
}
