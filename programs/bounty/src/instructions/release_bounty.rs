extern crate solana_sdk;
use crate::state::Bounty;
use crate::utils::{BlizzardError, BOUNTY_SEED};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};
use solana_sdk::pubkey::Pubkey;
use std::mem::size_of;

#[derive(Accounts)]
#[instruction(holder_address: Pubkey)]
pub struct ReleaseEscrow<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(seeds = [BOUNTY_SEED.as_bytes(), owner.key().to_bytes().as_ref()], bump, space = 8 + size_of::<Bounty>())]
    pub bounty: Account<'info, Bounty>,

    pub holder_address: Pubkey,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,

    #[account(constraint = escrow.key().eq(&bounty.escrow))]
    pub escrow: Account<'info, TokenAccount>,
}

impl ReleaseEscrow<'_> {
    pub fn release(&mut self, holder_address: &Pubkey) -> Result<()> {
        if self.bounty.is_owner(&self.owner.key()) {
            // find the account associated with holder_address
            // FIX ME: Account::find doesn't exist
            let holder_account = Account::find(holder_address, &self.system_program)?;
            transfer(
                CpiContext::new(
                    self.token_program.to_account_info(),
                    Transfer {
                        from: self.escrow.to_account_info(),
                        to: holder_account.to_account_info(),
                        authority: self.owner.to_account_info(),
                    },
                ),
                // FIX ME: owner.bump not existent
                &self.owner.bump(),
            )?;
            Ok(())
        } else {
            Err(BlizzardError::NotAuthToReleaseEscrow.into())
        }
    }
}

pub fn handler(ctx: Context<ReleaseEscrow>) -> Result<()> {
    let owner = &ctx.accounts.owner;
    let bounty = &ctx.accounts.bounty;
    let holder_address = &ctx.accounts.holder_address;

    if !bounty.is_owner(&owner.key()) {
        return Err(BlizzardError::NotAuthToReleaseEscrow.into());
    } else {
        // create an instance of ReleaseEscrow
        let mut release_escrow = ReleaseEscrow {
            owner: ctx.accounts.owner,
            escrow: ctx.accounts.escrow,
            bounty: ctx.accounts.bounty,
            holder_address: ctx.accounts.holder_address,
            //FIX ME: system_program + token_program not available
            system_program: ctx.program_id.system_program,
            token_program: ctx.program_id.token_program,
        };

        // release escrow
        release_escrow.release(&holder_address)?;

        Ok(())
    }
}
