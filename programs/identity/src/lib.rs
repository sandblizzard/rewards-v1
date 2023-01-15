use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod utils;
pub use crate::instructions::*;
pub use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod identity {
    use super::*;

    /// initialize
    ///
    /// Allows the first RELAYER to be initialize
    pub fn initialize(ctx: Context<CreateUser>) -> Result<()> {
        Ok(())
    }

    /// relay_create_user
    ///
    /// only RELAYER can relay
    pub fn relay_create_user(ctx: Context<CreateUser>) -> Result<()> {
        Ok(())
    }
}
