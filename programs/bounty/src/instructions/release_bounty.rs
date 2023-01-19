use crate::state::Bounty;
use crate::utils::{BlizzardError, BOUNTY_SEED};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

#[derive(Accounts)]
#[instruction(holder_address: Pubkey)]
pub struct ReleaseEscrow<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut, seeds = [BOUNTY_SEED.as_bytes(), owner.key().to_bytes().as_ref()], bump)]
    pub bounty: Account<'info, Bounty>,

    #[account(constraint = escrow.key().eq(&bounty.escrow))]
    pub escrow: Account<'info, TokenAccount>,

    /// FIXME: validate me
    pub receiver_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<ReleaseEscrow>, amount: u64) -> Result<()> {
    let owner = &ctx.accounts.owner;
    let bounty = &ctx.accounts.bounty;

    if !bounty.is_owner(&owner.key()) {
        return Err(BlizzardError::NotAuthToReleaseEscrow.into());
    } else {
        // create an instance of ReleaseEscrow
        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.escrow.to_account_info(),
                    to: ctx.accounts.receiver_account.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }
}
