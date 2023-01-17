pub use crate::state::user;
pub use crate::utils::*;
use anchor_lang::{
    prelude::*,
};
use std::mem::size_of;

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize)]
pub enum ProfileType {
    github,
}

#[derive(Accounts)]
#[instruction(user_address: Pubkey,profile: String, profile_type: String)]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer=payer,
        seeds = [
            REWARD_SEED.as_bytes(),
            user_address.key().to_bytes().as_ref(),
            profile.as_bytes(),
            profile_type.as_bytes()
        ],
        bump,
        space = 8 + size_of::<user::UserProfile>()
    )]
    pub user_account: Account<'info, user::UserProfile>,

    pub system_program: Program<'info, System>,
}

/// CreateUser handler is responsible for linking a user address and a profile
///
/// # args
/// * user_address: is the wallet address
/// * profile: is the social media profile
/// * profile_type: is the type of social media profile such as github
pub fn handler(
    ctx: Context<CreateUser>,
    _user_address: &Pubkey,
    _profile: String,
    _profile_type: String,
) -> Result<()> {
    let _user_account = &ctx.accounts.user_account;

    // FIXME
    //user_account.initialize();
    Result::Ok(())
}
