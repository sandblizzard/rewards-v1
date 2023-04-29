use crate::state::*;
use crate::utils::BOUNTY_SEED;
use anchor_lang::prelude::*;
use std::mem::size_of;

#[derive(Accounts)]
#[instruction(domain_type: String,platform: String,repo: String,sub_domain: String)]
pub struct CreateDomain<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(mut)]
    pub protocol: Account<'info, Protocol>,

    #[account(init,
    payer=creator,
    seeds=[
        BOUNTY_SEED.as_bytes(),
        platform.as_bytes(),
        sub_domain.as_bytes(),
        domain_type.as_bytes(),
        repo.as_bytes(),
    ],
    bump,
    space =8+size_of::<domain::Domain>(),
    )]
    pub domain: Account<'info, domain::Domain>,

    pub system_program: Program<'info, System>,
}

/// Initialize protocol
pub fn handler(
    ctx: Context<CreateDomain>,
    domain_type: String,
    platform: String,
    repo: String,
    sub_domain: String,
) -> Result<()> {
    let domain = &mut ctx.accounts.domain;
    let creator = &ctx.accounts.creator.key();

    // initialize the domain
    domain
        .initialize(&domain_type, &sub_domain, &platform, &repo, creator)
        .unwrap();
    Ok(())
}
