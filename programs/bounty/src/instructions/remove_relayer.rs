use crate::state::{Protocol, Relayer};
use crate::utils::BOUNTY_SEED;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(relayer_address: Pubkey)]
pub struct RemoveRelayer<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [
            BOUNTY_SEED.as_bytes()
        ],
        bump = protocol.bump,
        constraint = signer.key() == protocol.owner
    )]
    pub protocol: Account<'info, Protocol>,

    #[account(
        seeds=[BOUNTY_SEED.as_bytes(), relayer.owner.key().to_bytes().as_ref()],
        bump = relayer.bump,
        constraint = relayer.active == true
    )]
    pub relayer: Account<'info, Relayer>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RemoveRelayer>) -> Result<()> {
    let relayer = &mut ctx.accounts.relayer;
    relayer.deactive()?;
    Ok(())
}
