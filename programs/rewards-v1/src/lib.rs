use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod utils;
pub use crate::instructions::*;
pub use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod rewards_v1 {
    use super::*;

    /// initialize
    ///
    /// Allows the first RELAYER to be initialize
    pub fn initialize(ctx: Context<CreateUser>) -> Result<()> {
        Ok(())
    }

    /// add_relayer
    ///
    /// Anyone can be added as a RELAYER
    pub fn add_relayer(ctx: Context<CreateUser>) -> Result<()> {
        Ok(())
    }

    /// exit_relayer
    ///
    /// OWNER is allowed to exit being a relayer
    pub fn exit_relayer(ctx: Context<CreateUser>) -> Result<()> {
        Ok(())
    }

    /// add_listening_area
    ///
    /// Allows anyone to register a new area to listen for bounties
    /// This can be github orgs, repos
    ///
    pub fn add_listening_area(ctx: Context<CreateUser>) -> Result<()> {
        Ok(())
    }

    /// remove_listening_area
    ///
    /// Allows SPECIFIC users to close down areas/domains
    ///
    /// This would happen if the relayer(s) discovers that the area is broken
    /// or if the creator don't want it to listen anymore
    ///
    /// After an area is closed the bounty creators can redeeem their proposed
    /// bounties
    pub fn remove_listening_area(ctx: Context<CreateUser>) -> Result<()> {
        Ok(())
    }

    /// relay_create_user
    ///
    /// only RELAYER can relay
    pub fn relay_create_user(ctx: Context<CreateUser>) -> Result<()> {
        Ok(())
    }

    // ==== Bounties =====

    /// create_bounty
    ///
    /// when the RELAYER observes a bounty it will relay and send a
    /// tx to the BOUNTY_CREATOR
    ///
    /// The bounty transaction must be presented to the BOUNTY_CREATOR
    /// The bounty is not active until the create_bounty transaction is
    /// signed by the BOUNTY_CREATOR
    pub fn create_bounty(ctx: Context<CreateUser>) -> Result<()> {
        Ok(())
    }

    /// cancel_bounty
    ///
    /// The bounty can be cancelled by both the RELAYER and the BOUNTY_CREATOR
    ///
    /// If the bounty is stale or closed the RELAYER can choose to close it
    pub fn cancel_bounty(ctx: Context<CreateUser>) -> Result<()> {
        Ok(())
    }

    /// payout_bounty
    ///
    /// After a bounty is completed the RELAYER or BOUNTY_CREATOR
    /// determines who completed the bounty
    ///
    /// The method will payout the bounty and upgrade the stats for the
    /// SOLVERS
    pub fn payout_bounty(ctx: Context<CreateUser>) -> Result<()> {
        Ok(())
    }
}
