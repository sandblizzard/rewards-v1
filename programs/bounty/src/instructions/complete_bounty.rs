use anchor_lang::{
    prelude::*,
    solana_program::system_instruction::{transfer, transfer_many},
    system_program::Transfer,
};
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{
    state::Bounty,
    state::{Protocol, Relayer},
    utils::{BlizzardError, BOUNTY_SEED},
};

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
        constraint = protocol.fee_collector.eq(&fee_collector.owner.key()),
        constraint = fee_collector.mint.eq(&bounty.mint.key())
    )]
    pub fee_collector: Box<Account<'info, TokenAccount>>,

    /// relayer that wants to complete the transaction
    /// validate the seeds
    #[account(
        seeds=[BOUNTY_SEED.as_bytes(), relayer.owner.key().to_bytes().as_ref()],
        bump = relayer.bump
    )]
    pub relayer: Box<Account<'info, Relayer>>,

    /// bounty to be completed
    /// FIXME
    #[account(mut)]
    pub bounty: Box<Account<'info, Bounty>>,

    #[account(
        mut,
        seeds = [
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
    let relayer = &ctx.accounts.relayer;
    let payer = &ctx.accounts.payer;
    let bounty = &mut ctx.accounts.bounty;

    if !(bounty.is_owner(&payer.key()) || relayer.is_owner(&payer.key())) {
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
        let bounty_payout = bounty.complete_bounty(solvers, &ctx.accounts.fee_collector)?;
        let escrow = &ctx.accounts.escrow;

        let one_bounty = bounty_payout.get(0).unwrap();
        msg!(
            "Transfer bounty: {:?}, escrow: {:?}",
            bounty.escrow.to_string(),
            escrow.key().to_string()
        );
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: escrow.to_account_info(),
                    to: one_bounty.clone().0,
                    authority: bounty.to_account_info(),
                },
                &[&bounty.seeds()],
            ),
            one_bounty.clone().1,
        )
        .unwrap();

        // bounty_payout.iter().for_each(|(solver, amount)| {
        //     anchor_spl::token::transfer(
        //         CpiContext::new_with_signer(
        //             ctx.accounts.token_program.to_account_info(),
        //             anchor_spl::token::Transfer {
        //                 from: escrow.to_account_info(),
        //                 to: solver.clone(),
        //                 authority: escrow.to_account_info(),
        //             },
        //             &[&[
        //                 bounty.key().to_bytes().as_ref(),
        //                 bounty.escrow_bump_array.as_ref(),
        //             ]],
        //         ),
        //         *amount,
        //     )
        //     .unwrap()
        // });
    }
    Ok(())
}
