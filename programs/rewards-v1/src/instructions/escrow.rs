use anchor_lang::{
    prelude::*,
    solana_program::{instruction, sysvar::instructions},
};

use borsh::{BorshDeserialize, BorshSerialize};

use crate::state::*;
use crate::UserProfile;


#[instruction]
pub struct EscrowInstruction {
    pub escrow_amount: u64,
    pub bounty_id: String,
}

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct EscrowState {
    pub user_profile: UserProfile,
    pub escrow_amount: u64,
    pub bounty_id: String,
    pub escrow_status: EscrowStatus,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum EscrowStatus {
    Pending,
    Released,
    Cancelled,
}

impl EscrowInstruction {
    pub fn create_escrow(ctx: Context<EscrowInstruction>) -> Result<()> {
        let user_profile = ctx.read(ctx.arguments.user_profile)?;
        let mut escrow_state = EscrowState {
            user_profile,
            escrow_amount: ctx.arguments.escrow_amount,
            bounty_id: ctx.arguments.bounty_id,
            escrow_status: EscrowStatus::Pending,
        };
        ctx.write(escrow_state)?;
        Ok(())
    }

    pub fn release_escrow(ctx: Context<EscrowInstruction>) -> Result<()> {
        let mut escrow_state = ctx.read(ctx.arguments.user_profile)?;
        if let EscrowStatus::Pending = escrow_state.escrow_status {
            escrow_state.escrow_status = EscrowStatus::Released;
            ctx.write(escrow_state)?;
        } else {
            return Err(Error::CustomError(2));
        }
        Ok(())
    }

    pub fn cancel_escrow(ctx: Context<EscrowInstruction>) -> Result<()> {
        let mut escrow_state = ctx.read(ctx.arguments.user_profile)?;
        if let EscrowStatus::Pending = escrow_state.escrow_status {
            escrow_state.escrow_status = EscrowStatus::Cancelled;
            ctx.write(escrow_state)?;
        } else {
            return Err(Error::CustomError(2));
        }
        Ok(())
    }
}
