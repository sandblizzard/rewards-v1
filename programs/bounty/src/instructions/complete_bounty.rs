use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{
    state::bounty_state::BountyState,
    state::Bounty,
    state::Denomination,
    state::Protocol,
    utils::{BlizzardError, BOUNTY_SEED, DENOMINATION_SEED},
};

/// Complete bounty as a bounty creator
#[derive(Accounts)]
pub struct CompleteBounty<'info> {
    /// only owners or relayers can complete bounties
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds= [
            BOUNTY_SEED.as_bytes()
        ],
        bump=protocol.bump,
    )]
    pub protocol: Box<Account<'info, Protocol>>,

    #[account(
        mut,
        constraint = protocol.key().eq(&fee_collector.owner.key()) @ BlizzardError::WrongProtocolFeeCollector,
        constraint = fee_collector.mint.eq(&bounty.mint.key())  @ BlizzardError::WrongFeeCollectorMint
    )]
    pub fee_collector: Box<Account<'info, TokenAccount>>,

    /// bounty denomination is the allowed denomination of a bounty
    /// it needs to be checked against the fee collector and the mint
    #[account(
        seeds = [
            BOUNTY_SEED.as_bytes(),
            DENOMINATION_SEED.as_bytes(),
            bounty.mint.key().to_bytes().as_ref()
        ],
        bump = bounty_denomination.bump,
        constraint = bounty_denomination.fee_collector.eq(&fee_collector.key()) @ BlizzardError::WrongDenominationFeeCollector,
        constraint = bounty_denomination.mint.eq(&bounty.mint.key()) @ BlizzardError::WrongDenominationMint
    )]
    pub bounty_denomination: Box<Account<'info, Denomination>>,

    /// bounty to be completed
    /// FIXME
    #[account(mut,
        constraint=bounty.state.eq(&BountyState::Created))
    ]
    pub bounty: Box<Account<'info, Bounty>>,

    #[account(
        mut,
        seeds = [
            BOUNTY_SEED.as_bytes(),
            bounty.key().to_bytes().as_ref()
        ],
        bump = bounty.escrow_bump,
        constraint = escrow.key().eq(&bounty.escrow),
        constraint = escrow.mint.eq(&bounty.mint)
    )]
    pub escrow: Box<Account<'info, TokenAccount>>,

    /// up to 4 receivers
    #[account(mut)]
    pub solver1: Account<'info, TokenAccount>,
    #[account(mut)]
    pub solver2: Option<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub solver3: Option<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub solver4: Option<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<CompleteBounty>) -> Result<()> {
    msg!("Complete bounty");
    let payer = &ctx.accounts.payer;
    let bounty = &mut ctx.accounts.bounty;

    if !(bounty.is_owner(&payer.key())) {
        return Err(BlizzardError::NotAuthToCompleteBounty.into());
    } else {
        // create receivers vec
        msg!("Derref solvers");
        let mut solvers = Vec::new();
        let s1 = &ctx.accounts.solver1;
        solvers.push(s1);
        let s2 = &ctx.accounts.solver2;
        let s3 = &ctx.accounts.solver3;
        let s4 = &ctx.accounts.solver4;

        if s2.is_some() {
            solvers.push(s2.as_ref().unwrap())
        }
        if s3.is_some() {
            solvers.push(s3.as_ref().unwrap())
        }
        if s4.is_some() {
            solvers.push(s4.as_ref().unwrap())
        }

        msg!("Complete bounty");
        let bounty_payout =
            bounty.complete_bounty(solvers, &ctx.accounts.fee_collector, &payer.key())?;
        let escrow = &ctx.accounts.escrow;

        msg!(
            "Transfer bounty: {:?}, escrow: {:?}, total amount {}, payouts {:?}",
            bounty.escrow.to_string(),
            escrow.key().to_string(),
            bounty.bounty_amount,
            bounty_payout.iter().map(|pay| pay.1).collect::<Vec<u64>>()
        );

        // payout bounty
        bounty_payout.iter().for_each(|(solver, amount)| {
            anchor_spl::token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    anchor_spl::token::Transfer {
                        from: escrow.to_account_info(),
                        to: solver.clone(),
                        authority: bounty.to_account_info(),
                    },
                    &[&bounty.signing_seeds()],
                ),
                *amount,
            )
            .unwrap()
        });
    }
    Ok(())
}
