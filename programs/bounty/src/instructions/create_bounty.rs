use crate::state::Bounty;
use crate::utils::{BONK_MINT, BOUNTY_SEED};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};
use std::mem::size_of;

/// Anyone can create a bounty for anything
///
/// If comes from a users <-> nft then the creator needs to be the signer not the
/// owner of the NFT
#[derive(Accounts)]
#[instruction(domain: String,sub_domain: String,id: String)]
pub struct CreateBounty<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        seeds = [
            BOUNTY_SEED.as_bytes().as_ref(),
            domain.as_bytes(),
            sub_domain.as_bytes(),
            id.as_bytes(),
        ],
        bump,
        space=8 + size_of::<Bounty>()
    )]
    pub bounty: Account<'info, Bounty>,

    /// Account to credit the user
    #[account(mut)]
    pub creator_account: Account<'info, TokenAccount>,

    /// mint to use
    /// Only bonk
    #[account(
        //constraint = mint.key().to_string().eq(BONK_MINT),
        constraint = creator_account.mint.eq(&mint.key()),
        constraint = escrow.mint.eq(&mint.key())
    )]
    pub mint: Account<'info, Mint>,

    /// Bounty escrow to transfer funds to
    #[account(
        init,
        payer = creator,
        seeds = [
            bounty.key().to_bytes().as_ref()
        ],
        bump,
        token::mint = mint,
        token::authority = bounty,
    )]
    pub escrow: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

/// create_bounty handler
///
/// ## args
/// *  domain: e.g. github,jira
/// * area: e.g. backend
/// * sub_domain: e.g. Sandblizzard,Microsoft
/// * id: e.g. 453423
pub fn handler(
    ctx: Context<CreateBounty>,
    domain: String,
    sub_domain: String,
    id: String,
    bounty_amount: u64,
) -> Result<()> {
    let creator = &ctx.accounts.creator;
    let creator_account = &ctx.accounts.creator_account;
    let escrow = &ctx.accounts.escrow;
    let token_program = &ctx.accounts.token_program;
    // initialize the bounty
    ctx.accounts
        .bounty
        .create_bounty(
            ctx.bumps.get("bounty").unwrap(),
            &creator.key(),
            &escrow.key(),
            &domain,
            &sub_domain,
            &id,
            bounty_amount,
            &ctx.accounts.mint.key(),
            &ctx.bumps.get("escrow").unwrap(),
        )
        .unwrap();

    // transfer the bounty amount to the escrow
    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: creator_account.to_account_info(),
                to: escrow.to_account_info(),
                authority: creator.to_account_info(),
            },
        ),
        bounty_amount,
    )
    .unwrap();

    Ok(())
}
