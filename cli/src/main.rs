use bounty::{accounts, instruction, state::Protocol, utils};
use std::{io::Error, ops::Deref, rc::Rc, str::FromStr};

use anchor_client::{
    anchor_lang::{system_program, AccountDeserialize, InstructionData, ToAccountMetas},
    solana_sdk::{
        feature_set::{spl_associated_token_account_v1_0_4, spl_associated_token_account_v1_1_0},
        program_pack::Pack,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
        signer::Signer,
    },
    *,
};
use anchor_spl::{
    self,
    token::{self},
};
use bounty_sdk::{accounts::get_relayer_pda, utils::get_key_from_env, *};
use clap::{Parser, Subcommand};
use home;
use log;
use spl_associated_token_account::{self, solana_program::account_info::AccountInfo};
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// program_id of program
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Init {},
    AddBountyDenomination {
        #[arg(short, long, value_parser)]
        mint: Option<Pubkey>,
    },
    DeactivateBountyDenomination {
        #[arg(short, long, value_parser)]
        mint: Option<Pubkey>,
    },
    CreateMint {
        #[arg(short, long, value_parser)]
        receiver: Option<String>,
    },
    AddRelayer {
        #[arg(short, long, value_parser)]
        relayer: Option<String>,
    },
}

pub fn load_keypair() -> Result<Keypair, Error> {
    let keypair_location = format!(
        "{}/.config/solana/id.json",
        home::home_dir().unwrap().to_str().unwrap()
    );
    log::debug!("[CLI] Try keypair location {} ", keypair_location);
    let payer = read_keypair_file(keypair_location).unwrap();
    Ok(payer)
}

///initialize_bounty_contract
///
/// sets up the fee collector, the nft mint
pub fn initialize_bounty_contract() {
    // get keypair from config.id
    // CONFIG
    let sand_token_mint = Pubkey::from_str("A3LTRAn8fvZW5kuGRAXB7Xr1VGqVuCQUn1RxWSAtsJFH").unwrap();
    let nft_collection = Pubkey::from_str("BXKro6nDX9y86rtGn6uh6K1rZUqENzsUHP6gAbdJj1NS").unwrap();
    let cluster = match get_key_from_env("CLUSTER").unwrap().as_str() {
        "devnet" => Cluster::Devnet,
        "mainnet-beta" => Cluster::Mainnet,
        "testnet" => Cluster::Testnet,
        _ => Cluster::Devnet,
    };

    let client = bounty_sdk::program::BountySdk::new(Some(cluster)).unwrap();

    client.initialize_contract(&sand_token_mint, &nft_collection)
}

/// add_bounty_denomination
pub fn add_bounty_denomination(mint: &Option<Pubkey>) {
    let mint = match mint {
        Some(mint) => mint,
        None => {
            log::error!("No mint provided");
            return;
        }
    };
    let cluster = match get_key_from_env("CLUSTER").unwrap().as_str() {
        "devnet" => Cluster::Devnet,
        "mainnet-beta" => Cluster::Mainnet,
        "testnet" => Cluster::Testnet,
        _ => Cluster::Devnet,
    };

    let client = bounty_sdk::program::BountySdk::new(Some(cluster)).unwrap();

    client.add_bounty_denomination(mint)
}

/// deactivate_bounty_denomination
///
/// deactivates a bounty denomination by setting the active flag to false
pub fn deactivate_bounty_denomination(mint: &Option<Pubkey>) {
    let mint = match mint {
        Some(mint) => mint,
        None => {
            log::error!("No mint provided");
            return;
        }
    };
    let cluster = match get_key_from_env("CLUSTER").unwrap().as_str() {
        "devnet" => Cluster::Devnet,
        "mainnet-beta" => Cluster::Mainnet,
        "testnet" => Cluster::Testnet,
        _ => Cluster::Devnet,
    };

    let client = bounty_sdk::program::BountySdk::new(Some(cluster)).unwrap();

    client.deactivate_bounty_denomination(mint)
}

pub fn add_relayer(relayer_address: &Option<String>) {
    let relayer_address = match relayer_address {
        Some(rec) => Pubkey::from_str(rec).unwrap(),
        None => {
            log::error!("Could not find relayer");
            return;
        }
    };

    let relayer_pda = get_relayer_pda(&relayer_address);

    let client = bounty_sdk::program::BountySdk::new(None).unwrap();

    client.add_relayer(&relayer_pda.0)
}

pub fn create_mint(receiver: &Option<String>) {
    let payer = load_keypair().unwrap();
    let payer_pk = payer.pubkey();
    let receiver_pk = match receiver {
        Some(rec) => Pubkey::from_str(rec).unwrap(),
        None => payer.pubkey(),
    };
    let token_program_id = anchor_spl::token::spl_token::ID;
    let mint = Keypair::new();
    let mint_space = anchor_spl::token::spl_token::state::Mint::LEN as u64;

    let initialize_mint_ix = anchor_spl::token::spl_token::instruction::initialize_mint(
        &token_program_id,
        &mint.pubkey(),
        &payer.pubkey(),
        None,
        6,
    )
    .unwrap();

    let payer_rc = Rc::new(payer);
    let client = anchor_client::Client::new(Cluster::Devnet, payer_rc);
    let program = client.program(token_program_id);
    let rent = program
        .rpc()
        .get_minimum_balance_for_rent_exemption(mint_space as usize)
        .unwrap();
    let create_mint_ix = anchor_client::solana_sdk::system_instruction::create_account(
        &payer_pk,
        &mint.pubkey(),
        rent,
        mint_space,
        &token_program_id,
    );

    let receiver_ata =
        anchor_spl::associated_token::get_associated_token_address(&receiver_pk, &mint.pubkey());

    let payer_ata =
        anchor_spl::associated_token::get_associated_token_address(&payer_pk, &mint.pubkey());

    // Create associate token address for PAYER
    let create_associated_account_for_payer_ix =
        spl_associated_token_account::instruction::create_associated_token_account(
            &payer_pk,
            &payer_pk,
            &mint.pubkey(),
            &token_program_id,
        );

    // Create associate token address for RECEIVER
    let create_associated_account_for_receiver_ix =
        spl_associated_token_account::instruction::create_associated_token_account(
            &payer_pk,
            &receiver_pk,
            &mint.pubkey(),
            &token_program_id,
        );

    let mint_to_ix = anchor_spl::token::spl_token::instruction::mint_to(
        &token_program_id,
        &mint.pubkey(),
        &payer_ata,
        &payer_pk,
        &[&payer_pk, &mint.pubkey()],
        1_000_000_000_000_000,
    )
    .unwrap();

    let transfer_to_receiver_ix = anchor_spl::token::spl_token::instruction::transfer(
        &token_program_id,
        &payer_ata,
        &receiver_ata,
        &payer_pk,
        &[&payer_pk],
        1_000_000_000_000_000,
    )
    .unwrap();

    match program
        .request()
        .instruction(create_mint_ix)
        .instruction(initialize_mint_ix)
        .instruction(create_associated_account_for_payer_ix)
        .instruction(create_associated_account_for_receiver_ix)
        .instruction(mint_to_ix)
        .instruction(transfer_to_receiver_ix)
        .signer(&mint)
        .send()
    {
        Ok(res) => log::info!(
            "Success {}. Mint address {}",
            res,
            mint.pubkey().to_string()
        ),
        Err(err) => log::error!("Failure. cause: {}", err.to_string()),
    };
}

fn main() {
    let _env = env_logger::init();
    let args = Args::parse();

    match &args.command {
        Some(Commands::Init {}) => {
            initialize_bounty_contract();
        }
        Some(Commands::AddBountyDenomination { mint }) => {
            add_bounty_denomination(mint);
        }
        Some(Commands::DeactivateBountyDenomination { mint }) => {
            deactivate_bounty_denomination(mint);
        }
        Some(Commands::CreateMint { receiver }) => create_mint(receiver),
        Some(Commands::AddRelayer { relayer }) => add_relayer(relayer),

        None => log::error!("Command not found"),
    }
}
