use crate::state::*;
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use std::mem::size_of;

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// creator is the owner of the protocol
    /// should become a smart wallet over time
    #[account(mut)]
    pub protocol_owner: Signer<'info>,

    #[account(
        init,
        payer=protocol_owner,
        seeds=[
            BOUNTY_SEED.as_bytes(), // only one protocol config
        ],
        space=8 + size_of::<protocol::Protocol>(),
        bump
    )]
    pub protocol: Account<'info, protocol::Protocol>,

    /// mint to be used to distribute rewards
    #[account(
        init,
        seeds=[
            BOUNTY_SEED.as_bytes(),
            "sand_mint".as_bytes(),
        ],
        bump,
        payer=protocol_owner,
        mint::decimals=9,
        mint::authority=sand_mint,
        mint::freeze_authority=sand_mint,

    )]
    pub sand_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

/// Initialize protocol
///
/// creates a new protocol config and sets who gets to
/// control it
pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let protocol = &mut ctx.accounts.protocol;

    // Initialize collection
    protocol
        .initialize(
            ctx.bumps.get("protocol").unwrap(),
            &ctx.accounts.protocol_owner.key(),
            &mut ctx.accounts.sand_mint.key(),
        )
        .unwrap();

    Ok(())
}
