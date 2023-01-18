use crate::state::*;
use crate::utils::BOUNTY_SEED;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use std::mem::size_of;

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// creator is the owner
    /// should become a smart wallet over time
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer=creator,
        seeds=[
            BOUNTY_SEED.as_bytes(), // only one protocol config
        ],
        space=8 + size_of::<protocol::Protocol>(),
        bump
    )]
    pub protocol: Account<'info, protocol::Protocol>,

    /// CHECK: Fee collector is an account that controls fees
    #[account()]
    pub fee_collector: AccountInfo<'info>,

    /// mint used for the collection
    pub collection: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
}

/// Initialize protocol
///
/// creates a new protocol config and sets who gets to
/// control it
pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let protocol = &mut ctx.accounts.protocol;
    let collection = &mut ctx.accounts.collection;
    // Initialize collection
    protocol
        .initialize(&ctx.accounts.creator.key(), &collection.key())
        .unwrap();

    Ok(())
}
