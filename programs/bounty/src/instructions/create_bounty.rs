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
pub struct CreateBounty<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        //constraint = mint.key().to_string().eq(BONK_MINT),
        constraint = creator_account.mint.eq(&mint.key()),
        constraint = escrow.mint.eq(&mint.key())
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = creator,
        seeds = [
            BOUNTY_SEED,
            id.to_le_bytes().as_ref(),
        ],
        bump,
        space=8 + size_of::<Bounty>()
    )]
    pub bounty: Box<Account<'info, Bounty>>,

    /// domain to attach the bounty to
    #[account(
        seeds=[
            BOUNTY_SEED,
            domain.data.platform.as_bytes(),
            domain.data.organization.as_bytes(),
            domain.data.team.as_bytes(),
            domain.data.domain_type.as_bytes(),
        ],
        bump=domain.bump,
        constraint = domain.active @BlizzardError::DomainNotActive,
    )]
    pub domain: Box<Account<'info, Domain>>,

    /// Account to credit the user
    #[account(mut)]
    pub creator_account: Account<'info, TokenAccount>,

    // todo: check seeds
    #[account(
        seeds = [
            BOUNTY_SEED,
            DENOMINATION_SEED,
            bounty_denomination.mint.to_bytes().as_ref()
        ],
        bump=bounty_denomination.bump,
        constraint = bounty_denomination.mint.eq(&mint.key()),
        constraint = bounty_denomination.active
    )]
    pub bounty_denomination: Box<Account<'info, Denomination>>,

    /// Bounty escrow to transfer funds to
    #[account(
        init,
        payer = creator,
        seeds = [
            BOUNTY_SEED,
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
    id: u64,
    external_id: String,
    title: String,
    description: String,
    ends_at: Option<i64>,
) -> Result<()> {
    let creator = &ctx.accounts.creator;
    let domain = &ctx.accounts.domain;
    let escrow: &Account<'_, TokenAccount> = &ctx.accounts.escrow;

    // initialize the bounty
    ctx.accounts
        .bounty
        .create_bounty(
            &ctx.bumps.bounty,
            &title,
            &description,
            &id,
            &external_id,
            &creator.key(),
            &escrow.key(),
            &domain.key(),
            &ctx.accounts.mint.key(),
            &ctx.bumps.escrow,
            ends_at,
        )
        .unwrap();

    // transfer the bounty amount to the escrow
    // transfer(
    //     CpiContext::new(
    //         token_program.to_account_info(),
    //         Transfer {
    //             from: creator_account.to_account_info(),
    //             to: escrow.to_account_info(),
    //             authority: creator.to_account_info(),
    //         },
    //     ),
    //     bounty_amount,
    // )
    // .unwrap();

    Ok(())
}
