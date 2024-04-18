use anchor_lang::prelude::*;
use anchor_spl::token::{accessor::mint, mint_to, Mint, MintTo, Token, TokenAccount};
use solana_program::native_token::Sol;

use crate::{
    complete_bounty, protocol_collector,
    state::{bounty_state::BountyState, Bounty, Denomination, Protocol},
    utils::{BlizzardError, BOUNTY_SEED},
    Relayer, Solver, TSolver,
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

pub fn get_solvers<'info>(
    solver1: &Account<'info, Solver>,
    solver2: Option<&Account<'info, Solver>>,
) -> Vec<Box<dyn TSolver>> {
    let mut solvers: Vec<Box<dyn TSolver>> = Vec::new();
    solvers.push(Box::new(solver1.clone().into_inner()));
    if solver2.is_some() {
        solvers.push(Box::new(solver2.unwrap().clone().into_inner()))
    }
    solvers
}

pub fn get_solver_token_accounts(
    solver_token_account_1: &TokenAccount,
    solver_token_account_2: &Option<TokenAccount>,
) -> Vec<TokenAccount> {
    let mut solvers = Vec::new();

    solvers.push(solver_token_account_1.clone());
    // if solver_token_account_2.is_some() {
    //     solvers.push(solver_token_account_2.unwrap().clone())
    // }
    solvers
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
    let bounty = &mut ctx.accounts.bounty;
    let payer = &ctx.accounts.payer;
    let protocol = &ctx.accounts.protocol;
    let sand_mint = &ctx.accounts.sand_mint;
    let fee_collector = &ctx.accounts.fee_collector;
    let escrow = &ctx.accounts.escrow;
    let token_program = &ctx.accounts.token_program;
    if !(bounty.is_owner(&payer.key())) {
        return Err(BlizzardError::NotAuthToCompleteBounty.into());
    } else {
        // create receivers vec
        msg!("Derref solvers");

        // let solver_token_account2 = match ctx.accounts.solver_token_account_2 {
        //     Some(solver) => Some(solver.to_account_info()),
        //     None => None,
        // };
        // let mut solvers = get_solvers(&ctx.accounts.solver1, ctx.accounts.solver2.as_ref());
        // let solver_token_accounts = get_solver_account_info(
        //     &ctx.accounts.solver_token_account_1.to_account_info(),
        //     solver_token_account2.as_ref(),
        // );
        // msg!("Complete bounty");
        // let bounty_payout = ctx.accounts.bounty.calculate_bounty_payout(
        //     &solver_token_accounts,
        //     &fee_collector,
        //     &payer.key,
        // )?;
        // let escrow = &escrow;

        // msg!(
        //     "Transfer bounty: {:?}, escrow: {:?}, total amount {}, payouts {:?}",
        //     ctx.accounts.bounty.escrow.to_string(),
        //     escrow.key().to_string(),
        //     ctx.accounts.bounty.bounty_amount,
        //     bounty_payout.iter().map(|pay| pay.1).collect::<Vec<u64>>()
        // );

        // // update claimable mining reward
        // let mining_reward = protocol.calculate_mining_reward(solvers.len(), sand_mint.decimals);
        // solvers
        //     .iter_mut()
        //     .for_each(|(solver)| solver.update_rewards(mining_reward).unwrap());

        // // update claimable bounty reward in a given mint

        // bounty_payout.iter().for_each(|(solver, amount)| {
        //     anchor_spl::token::transfer(
        //         CpiContext::new_with_signer(
        //             token_program.to_account_info(),
        //             anchor_spl::token::Transfer {
        //                 from: escrow.to_account_info(),
        //                 to: solver.clone(),
        //                 authority: bounty.to_account_info(),
        //             },
        //             &[&bounty.signing_seeds()],
        //         ),
        //         *amount,
        //     )
        //     .unwrap()
        // });
    }
    Ok(())
}
