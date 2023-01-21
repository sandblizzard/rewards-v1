use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub mod utils;

pub use instructions::*;

declare_id!("HELsfudvMchbrYGCFiBPwGp5eXt2kwoPLKDVByy4cLzd");

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

    /// complete_bounty
    ///
    /// Try to complete bounty
    pub fn complete_bounty(ctx: Context<CompleteBounty>) -> Result<()> {
        complete_bounty::handler(ctx)
    }

    pub fn add_relayer(ctx: Context<AddRelayer>, relayer_address: Pubkey) -> Result<()> {
        add_relayer::handler(ctx, relayer_address)
    }
}
