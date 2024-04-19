use crate::{
    utils::{BlizzardError, BOUNTY_SEED},
    Protocol, Solver, TSolver,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};
#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds= [
            BOUNTY_SEED
        ],
        bump=protocol.bump,
    )]
    pub protocol: Box<Account<'info, Protocol>>,

    #[account(
        mut,
        seeds = [
            BOUNTY_SEED,
            solver.owner.key().to_bytes().as_ref(),
        ],
        bump = solver.bump,
    )]
    pub solver: Account<'info, Solver>,

    /// token pda
    #[account(
        mut,
        constraint = solver_token_account.owner.eq(&signer.key()) @ BlizzardError::WrongSolverTokenAccountOwner,
    )]
    pub solver_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = mint.mint_authority.unwrap().eq(&protocol.key()) @ BlizzardError::WrongProtocolMintAuthority,
    )]
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<ClaimReward>) -> Result<()> {
    let solver = &mut ctx.accounts.solver;
    let protocol = &ctx.accounts.protocol;

    let claimable_rewards = solver.claimable_rewards;
    if claimable_rewards == 0 {
        return Err(BlizzardError::NoClaimableReward.into());
    }
    // mint reward
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.solver_token_account.to_account_info(),
                authority: ctx.accounts.protocol.to_account_info(),
            },
            &[&protocol.seeds()],
        ),
        claimable_rewards,
    )?;

    solver.claim_rewards()?;
    Ok(())
}
