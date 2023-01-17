use crate::state::*;
use crate::utils::REWARD_SEED;
use anchor_lang::prelude::*;
use std::mem::size_of;

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// creator becomes the owner
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer=creator,
        seeds=[
            REWARD_SEED.as_bytes(),
        ],
        space=8 + size_of::<protocol::Protocol>(),
        bump
    )]
    pub protocol: Account<'info, protocol::Protocol>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let protocol = &mut ctx.accounts.protocol;

    protocol.initialize(&ctx.accounts.creator.key()).unwrap();

    Ok(())
}
