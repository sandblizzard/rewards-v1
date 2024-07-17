use crate::state::{Protocol, Relayer};
use crate::utils::BOUNTY_SEED;
use anchor_lang::prelude::*;
use std::mem::size_of;

#[derive(Accounts)]
#[instruction(relayer_address: Pubkey)]
pub struct AddRelayer<'info> {
    #[account(
        mut,
        // only protocol owner is allowed to add relayers
        constraint = signer.key() == protocol.owner
    )]
    pub signer: Signer<'info>,

    #[account(
        seeds = [
            BOUNTY_SEED
        ],
        bump = protocol.bump
    )]
    pub protocol: Account<'info, Protocol>,

    #[account(
        init,
        payer = signer,
        seeds = [
            BOUNTY_SEED,
            relayer_address.to_bytes().as_ref(),
        ],
        space = 8 + size_of::<Relayer>(),
        bump,
    )]
    pub relayer: Account<'info, Relayer>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AddRelayer>, relayer_address: Pubkey) -> Result<()> {
    let relayer = &mut ctx.accounts.relayer;
    relayer.initialize(&relayer_address, &ctx.bumps.relayer)?;
    Ok(())
}
