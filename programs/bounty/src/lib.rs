use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub mod utils;

pub use instructions::*;
pub use state::*;
declare_id!("5Hwbrh6QMrHvBNZfYXmsktWtfohcSSCMaC5Er9ErwNoQ");

#[program]
pub mod bounty {

    use super::*;

    /// initialize
    ///
    /// - Initializes the protocol
    /// - creates the bounty mint
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    /// register solver
    ///
    /// Register solver for the first time in the protocol
    /// This will create a new solver account and a token account
    pub fn register_solver(ctx: Context<RegisterSolver>) -> Result<()> {
        register_solver::handler(ctx)
    }

    /// add bounty denomination
    /// it
    pub fn add_bounty_denomination(ctx: Context<AddBountyDenomination>) -> Result<()> {
        add_bounty_denomination::handler(ctx)
    }

    /// deactivate bounty denomination
    pub fn deactivate_bounty_denomination(
        ctx: Context<DeactivateBountyDenomination>,
    ) -> Result<()> {
        deactivate_bounty_denomination::handler(ctx)
    }

    /// create_bounty
    ///
    /// creates a bounty
    pub fn create_bounty(ctx: Context<CreateBounty>, id: u64, bounty_amount: u64) -> Result<()> {
        create_bounty::handler(ctx, id, bounty_amount)
    }

    /// complete_bounty
    ///
    /// Try to complete bounty
    pub fn complete_bounty<'info>(
        ctx: Context<'_, '_, '_, 'info, CompleteBountyAsCreator<'info>>,
    ) -> Result<()> {
        complete_bounty::handler(ctx)
    }

    /// complete_bounty
    ///
    /// Try to complete bounty
    pub fn complete_bounty_as_relayer<'info>(
        ctx: Context<'_, '_, '_, 'info, CompleteBountyAsRelayer<'info>>,
    ) -> Result<()> {
        complete_bounty_as_relayer::handler(ctx)
    }

    pub fn add_relayer(ctx: Context<AddRelayer>, relayer_address: Pubkey) -> Result<()> {
        add_relayer::handler(ctx, relayer_address)
    }
    pub fn remove_relayer(ctx: Context<RemoveRelayer>) -> Result<()> {
        remove_relayer::handler(ctx)
    }

    /// create domain
    pub fn create_domain(
        ctx: Context<CreateDomain>,
        domain_type: String,
        platform: String,
        organization: String,
        team: String,
    ) -> Result<()> {
        create_domain::handler(ctx, domain_type, platform, organization, team)
    }

    /// deactivate domain
    pub fn deactivate_domain(ctx: Context<DeactivateDomain>) -> Result<()> {
        deactivate_domain::handler(ctx)
    }

    /// Claim rewards
    ///
    /// Claim rewards for the bounty
    pub fn claim_rewards(ctx: Context<ClaimReward>) -> Result<()> {
        claim::handler(ctx)
    }
}
