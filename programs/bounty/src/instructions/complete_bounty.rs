use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};


use crate::{
    bounty_state::calculate_bounty_payout,
    state::{bounty_state::BountyState, Bounty, Protocol},
    utils::{BlizzardError, BOUNTY_SEED}, Solver, TSolver,
};

#[derive(Accounts)]
pub struct CompleteBountyAsCreator<'info> {
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
    pub escrow: Box<Account<'info, TokenAccount>>,

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
}

/// Complete bounty as a bounty creator
#[derive(Accounts)]
pub struct CompleteBounty<'info> {
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
    pub escrow: Box<Account<'info, TokenAccount>>,

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
}

pub fn get_solver_account_info<'a>(
    solver1: &Account<'a, TokenAccount>,
    solver2: &Option<Account<'a, TokenAccount>>,
) -> Vec<AccountInfo<'a>> {
    let mut solvers = Vec::new();
    solvers.push(solver1.to_account_info().clone());
    if solver2.is_some() {
        solvers.push(solver2.as_ref().unwrap().to_account_info())
    }
    solvers
}

impl<'info> CompleteBounty<'info> {
    pub fn get_solver_token_accounts(&mut self) -> Vec<&Account<'info, TokenAccount>> {
        let mut solvers = Vec::new();

        solvers.push(&self.solver_token_account_1);
        if self.solver_token_account_2.is_some() {
            solvers.push(self.solver_token_account_2.as_ref().unwrap())
        }
        solvers
    }

    pub fn get_solvers(&mut self) -> Vec<Box<dyn TSolver>> {
        let mut solvers: Vec<Box<dyn TSolver>> = Vec::new();
        solvers.push(Box::new(self.solver1.clone().into_inner()));
        if self.solver2.is_some() {
            solvers.push(Box::new(self.solver2.clone().unwrap().into_inner()))
        }
        solvers
    }
}

pub fn handler(ctx: Context<CompleteBountyAsCreator>) -> Result<()> {
    msg!("Complete bounty");
    let payer = &ctx.accounts.payer;
    let protocol = &ctx.accounts.protocol;
    let sand_mint = &ctx.accounts.sand_mint;
    let fee_collector = &ctx.accounts.fee_collector;

    let bounty = &mut ctx.accounts.bounty;
    if !(bounty.is_owner(&payer.key())) {
        return Err(BlizzardError::NotAuthToCompleteBounty.into());
    } else {
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
    }
    Ok(())
}
