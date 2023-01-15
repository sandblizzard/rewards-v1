use super::*;
use borsh::{BorshDeserialize, BorshSerialize};
use crate::state::*;

// use crate::borsh::{BorshDeserialize, BorshSerialize};

use anchor_lang::{
    prelude::*,
    // solana_program::nft_mint
};

#[derive(Accounts)]
pub struct NftMint<'info> {
    #[account(mut)]
    pub user_account: Account<'info, user::UserProfile>,
    pub system_program: Program<'info, System>,
}

/// NftMint handler is responsible for minting an NFT to a user's address
///
pub fn handler(ctx: Context<NftMint>) -> Result<()> {
    let user_address = ctx.accounts.user_account.key();
    let account_data = ctx.accounts.user_account.try_get_data()?; 
    
    // FIX ME - mint nft with above wallet address & github name
    // let user_data: user::UserProfile = BorshDeserialize::deserialize(account_data.as_mut_slice())?;
    // let user_metadata = BorshSerialize::serialize(&user_data)?;

    Ok(())
}
