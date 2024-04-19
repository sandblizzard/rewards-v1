

use crate::state::protocol_collector::TSolver;

use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount},
};



use crate::{
    bounty_state::calculate_bounty_payout,
    get_solver_account_info,
    state::{bounty_state::BountyState, Bounty, Protocol, Relayer},
    utils::{BlizzardError, BOUNTY_SEED},
    Solver,
};

#[derive(Accounts)]
pub struct CompleteBountyAsRelayer<'info> {
    /// only owners or relayers can complete bounties
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds= [
            BOUNTY_SEED
        ],
        bump=protocol.bump,
    )]
    pub protocol: Box<Account<'info, Protocol>>,

    #[account(
        mut,
        constraint = sand_mint.mint_authority.unwrap().eq(&protocol.key())
    )]
    pub sand_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = protocol.key().eq(&fee_collector.owner.key()) @ BlizzardError::WrongProtocolFeeCollector,
        constraint = fee_collector.mint.eq(&bounty.mint.key())  @ BlizzardError::WrongFeeCollectorMint
    )]
    pub fee_collector: Box<Account<'info, TokenAccount>>,

    /// bounty to be completed
    /// FIXME
    #[account(mut,
        constraint=bounty.state.eq(&BountyState::Created))
    ]
    pub bounty: Box<Account<'info, Bounty>>,

    #[account(
        mut,
        seeds = [
            BOUNTY_SEED,
            bounty.key().to_bytes().as_ref()
        ],
        bump = bounty.escrow_bump,
        constraint = escrow.key().eq(&bounty.escrow),
        constraint = escrow.mint.eq(&bounty.mint)
    )]
    pub escrow: Account<'info, TokenAccount>,

    #[account(mut)]
    pub solver_token_account_1: Account<'info, TokenAccount>,
    #[account(mut)]
    pub solver_token_account_2: Option<Account<'info, TokenAccount>>,

    /// up to 4 receivers
    #[account(mut)]
    pub solver1: Account<'info, Solver>,
    #[account(mut)]
    pub solver2: Option<Account<'info, Solver>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,

    /// relayer that wants to complete the transaction
    /// validate the seeds
    #[account(
        seeds=[
            BOUNTY_SEED,
            relayer.owner.key().to_bytes().as_ref()
        ],
        bump = relayer.bump,
        constraint = relayer.active @ BlizzardError::AccountNotActive,
        constraint = relayer.owner.key() == payer.key() @ BlizzardError::AccountIsNotSigner
    )]
    pub relayer: Box<Account<'info, Relayer>>,
}

pub fn handler(ctx: Context<CompleteBountyAsRelayer>) -> Result<()> {
    msg!("Complete bounty as relayer");
    let payer = &ctx.accounts.payer;
    let protocol = &ctx.accounts.protocol;
    let sand_mint = &ctx.accounts.sand_mint;
    let fee_collector = &ctx.accounts.fee_collector;

    let relayer = &ctx.accounts.relayer;
    let bounty = &mut ctx.accounts.bounty;
    if !(bounty.is_owner(&payer.key()) || relayer.is_owner(&payer.key())) {
        return Err(BlizzardError::NotAuthToCompleteBounty.into());
    }

    msg!("Derref solvers");

    let solver_token_accounts = get_solver_account_info(
        &ctx.accounts.solver_token_account_1,
        &ctx.accounts.solver_token_account_2,
    );
    let bounty_payout_proto = calculate_bounty_payout(
        &bounty.bounty_amount.clone(),
        &solver_token_accounts,
        &fee_collector.to_account_info(),
    )?;

    bounty_payout_proto.iter().for_each(|(solver, amount)| {
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.escrow.to_account_info(),
                    to: solver.clone(),
                    authority: bounty.as_mut().clone().to_account_info(),
                },
                &[&bounty.as_mut().clone().signing_seeds()],
            ),
            *amount,
        )
        .unwrap()
    });

    // // update claimable mining reward
    let mining_reward =
        protocol.calculate_mining_reward(solver_token_accounts.len(), sand_mint.decimals);
    ctx.accounts.solver1.update_rewards(mining_reward)?;
    if ctx.accounts.solver2.is_some() {
        ctx.accounts
            .solver2
            .as_mut()
            .unwrap()
            .update_rewards(mining_reward)?;
    }

    bounty.complete_bounty(ctx.accounts.payer.key())?;

    Ok(())
}
