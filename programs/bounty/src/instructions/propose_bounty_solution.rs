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
#[instruction()]
pub struct ProposeBountySolution<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            BOUNTY_SEED,
            bounty.id_bytes.as_ref(),
        ],
        bump,
    )]
    pub bounty: Box<Account<'info, Bounty>>,

    #[account(
        init,
        payer = signer,
        seeds = [
            BOUNTY_SEED,
            bounty.key().to_bytes().as_ref(),
            signer.key().as_ref(),
        ],
        bump,
        space = 8 + size_of::<BountySolution>(),
    )]
    pub bounty_solution: Account<'info, BountySolution>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

/// Donate to a bounty
pub fn handler(ctx: Context<ProposeBountySolution>, solution: String) -> Result<()> {
    let bounty_solution = &mut ctx.accounts.bounty_solution;
    let bounty_solution_bump = &ctx.bumps.bounty_solution;

    let id = (ctx.accounts.bounty.solver_solutions.len() as u64) + 1;
    bounty_solution
        .initialize(
            bounty_solution_bump,
            id,
            &ctx.accounts.bounty.key(),
            &ctx.accounts.signer.key(),
            &solution,
        )
        .unwrap();

    let bounty = &mut ctx.accounts.bounty;
    // initialize the bounty
    bounty
        .propose_solution(ctx.accounts.signer.key, &ctx.accounts.bounty_solution.key())
        .unwrap();

    // transfer the bounty amount to the escrow
    // transfer(
    //     CpiContext::new(
    //         token_program.to_account_info(),
    //         Transfer {
    //             from: ctx.accounts.donater_account.to_account_info(),
    //             to: ctx.accounts.escrow.to_account_info(),
    //             authority: ctx.accounts.signer.to_account_info(),
    //         },
    //     ),
    //     amount,
    // )
    // .unwrap();

    Ok(())
}
