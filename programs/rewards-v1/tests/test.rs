pub mod lib;
pub use lib::ProgramManager;

// pub mod state;
// use state::*;

// use anchor_lang::prelude::*;

// use tokio::test;
// use borsh::{BorshDeserialize, BorshSerialize};



async fn test_manage_relayers() {
    let program_manager = ProgramManager::init();
    println!("Hello there");
    // test_try_get_data();
}

/*
#[tokio::test]
#[derive(Accounts)]
struct CreateUser<'info> {
    #[account(mut)]
    user_account: Account<'info, UserProfile>,
    pub system_program: Program<'info, System>,
}

#[tokio::test]
async fn test_try_get_data() {
    let wallet_address = Pubkey::new_rand();
    let user_profile = "Axelofwar";
    let social_media = "https://github.com/axelofwar";
    let nft_mint = Pubkey::new_rand();

    let mut user_account = Account::new(UserProfile::default(), 1);
    let ctx = Context::new(vec![user_account.clone()], vec![wallet_address]);
    let mut user_profile = ctx.get_account::<UserProfile>(0).unwrap();
    user_profile.initialize(&wallet_address, &user_profile, &social_media, &nft_mint).unwrap();
    let data = user_profile.try_get_data();
    assert!(data.is_ok());
}
*/

