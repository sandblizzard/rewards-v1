use crate::state::*;
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};
use std::mem::size_of;

/// Anyone can create a bounty for anything
///
/// If comes from a users <-> nft then the creator needs to be the signer not the
/// owner of the NFT
#[derive(Accounts)]
#[instruction( id: u64)]
pub struct DonateToBounty<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            BOUNTY_SEED,
            bounty.id_bytes.as_ref(),
        ],
        bump,
    )]
    pub bounty: Box<Account<'info, Bounty>>,

    /// Account to credit the user
    #[account(
        mut,
        constraint = donater_token_account.mint.eq(&bounty.mint)
    )]
    pub donater_token_account: Account<'info, TokenAccount>,

    /// Bounty escrow to transfer funds to
    #[account(
        mut,
        seeds = [
            BOUNTY_SEED,
            bounty.key().to_bytes().as_ref()
        ],
        bump=bounty.escrow_bump,
    )]
    pub escrow: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

/// Donate to a bounty
pub fn handler(ctx: Context<DonateToBounty>, amount: u64) -> Result<()> {
    let token_program = &ctx.accounts.token_program;
    let bounty = &mut ctx.accounts.bounty;
    // initpialize the bounty
    bounty
        .donate_to_bounty(ctx.accounts.payer.key, amount)
        .unwrap();

    // transfer the bounty amount to the escrow
    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.donater_token_account.to_account_info(),
                to: ctx.accounts.escrow.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        ),
        amount,
    )
    .unwrap();

    Ok(())
}
