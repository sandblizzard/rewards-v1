use crate::utils::BOUNTY_SEED;
use crate::{Protocol, Solver};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use std::mem::size_of;

#[derive(Accounts)]
pub struct RegisterSolver<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub protocol: Account<'info, Protocol>,

    #[account(
        init,
        payer = signer,
        space = 8 + size_of::<Solver>(),
        seeds = [
            BOUNTY_SEED,
            signer.key().as_ref(),
            ],
        bump,
    )]
    pub solver_account: Account<'info, Solver>,

    // sand mint
    #[account(
        mut,
        constraint = sand_mint.key().eq(&protocol.sand_mint),
    )]
    pub sand_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = sand_mint,
        associated_token::authority = signer
    )]
    pub solver_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

/// Register solver for the first time in the protocol
/// This will create a new solver account and a token account
pub fn handler(ctx: Context<RegisterSolver>) -> Result<()> {
    let solver = &mut ctx.accounts.solver_account;

    solver.initialize(
        &ctx.accounts.signer.key(),
        &ctx.bumps.solver_account,
        ctx.accounts.sand_mint.key(),
    )?;
    Ok(())
}
