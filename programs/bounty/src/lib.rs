use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub mod utils;

pub use instructions::*;

declare_id!("4F4GVSFk1AidP7mgtoFuJRxcVrD3jtmJmpaZwTjq1E2B");

#[program]
pub mod bounty {
    use super::*;

    /// initialize
    ///
    /// Initializes the protocol and sets the config
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    /// create_bounty
    ///
    /// creates a bounty
    pub fn create_bounty(
        ctx: Context<CreateBounty>,
        domain: String,
        sub_domain: String,
        id: String,
        bounty_amount: u64,
    ) -> Result<()> {
        create_bounty::handler(ctx, domain, sub_domain, id, bounty_amount)
    }
}
