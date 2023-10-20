use crate::state::*;
use crate::utils::BOUNTY_SEED;
use anchor_lang::prelude::*;
use std::mem::size_of;

#[derive(Accounts)]
#[instruction( 
    domain_type: String,
    platform: String,
    organization: String,
    team: String
)]
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
        organization.as_bytes(),
        team.as_bytes(),
        domain_type.as_bytes(),
    ],
    bump,
    space =8+size_of::<domain::Domain>(),
    )]
    pub domain: Account<'info, domain::Domain>,

    pub system_program: Program<'info, System>,
}

/// Initialize protocol
///
/// * Args
pub fn handler(
    ctx: Context<CreateDomain>,
    domain_type: String,
    platform: String,
    organization: String,
    team: String,
) -> Result<()> {
    let domain_account = &mut ctx.accounts.domain;
    let creator = &ctx.accounts.creator.key();

    // initialize the domain
    domain_account
        .initialize(&domain_type, &organization, &team, &platform, creator)
        .unwrap();
    Ok(())
}
