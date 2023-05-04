use crate::state;
use crate::utils::BOUNTY_SEED;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DeactivateDomain<'info> {
    // signer
    pub signer: Signer<'info>,

    // domain
    #[account(
        mut,
        seeds = [
            BOUNTY_SEED.as_bytes(),
            domain.domain.platform.as_bytes(),
            domain.domain.sub_domain.as_bytes(),
            domain.domain.domain_type.as_bytes(),
        ],
        bump = domain.bump,
        constraint = domain.active,
        constraint = domain.owner == *signer.key
    )]
    pub domain: Account<'info, state::Domain>,

    pub system_program: Program<'info, System>,
}

/// deactivate domain
pub fn handler(ctx: Context<DeactivateDomain>) -> Result<()> {
    let domain = &mut ctx.accounts.domain;
    domain.deactivate()?;
    Ok(())
}
