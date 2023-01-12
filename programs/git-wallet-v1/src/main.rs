use anchor_client::{
    anchor::{mint, transfer},
    Client,
};
use solana_sdk::pubkey::Pubkey;
use serde_json::json;
use std::io;

fn main() {
    // Get the GitHub username from the user
    println!("Enter your GitHub username:");
    let mut github_username = String::new();
    io::stdin().read_line(&mut github_username).unwrap();
    let github_username = github_username.trim();

    // Get the Solana wallet address from the user
    println!("Enter your Solana wallet address:");
    let mut solana_wallet_address = String::new();
    io::stdin().read_line(&mut solana_wallet_address).unwrap();
    let solana_wallet_address = solana_wallet_address.trim();

    // Replace with the actual values for your own use case
    let anchor_contract_address = Pubkey::from_str("anchor_contract_address").unwrap();
    let mint_authority = Pubkey::from_str("mint_authority_pubkey").unwrap();
    let client = Client::new("https://testnet.solana.com");

    // Build the metadata for the NFT
    let metadata = json!({ "github_username": github_username }).to_string();

    // Mint the NFT
    let nft_id = mint(
        &client,
        &anchor_contract_address,
        &mint_authority,
        metadata.as_bytes(),
    )
    .expect("Minting failed");
    println!("NFT minted with ID: {}", nft_id);

    // Convert the wallet address to the correct format
    let wallet_pubkey = Pubkey::from_str(solana_wallet_address).unwrap();

    // Transfer the NFT to the wallet
    transfer(&client, &anchor_contract_address, &nft_id, &wallet_pubkey)
        .expect("Transfer failed");
    println!("NFT transferred to wallet!");
}
