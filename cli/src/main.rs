use bounty::{accounts, instruction, utils};
use std::{io::Error, rc::Rc, str::FromStr};

use anchor_client::{
    anchor_lang::{system_program, InstructionData, ToAccountMetas},
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
    self, associated_token,
    token::{self, Token},
};
use clap::{Parser, Subcommand};
use home;
use log;
use spl_associated_token_account;
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
    let sand_token_mint = Pubkey::from_str("A3LTRAn8fvZW5kuGRAXB7Xr1VGqVuCQUn1RxWSAtsJFH").unwrap();
    let fee_collector = Pubkey::from_str("CNY467c6XURCPjiXiKRLCvxdRf3bpunagYTJpr685gPv").unwrap();
    let nft_collection = Pubkey::from_str("BXKro6nDX9y86rtGn6uh6K1rZUqENzsUHP6gAbdJj1NS").unwrap();
    let bounty_program_id = bounty::id();
    let payer = load_keypair().unwrap();

    let sand_token_account = Pubkey::find_program_address(
        &[utils::BOUNTY_SEED.as_bytes(), sand_token_mint.as_ref()],
        &bounty_program_id,
    );
    let protocol =
        Pubkey::find_program_address(&[utils::BOUNTY_SEED.as_bytes()], &bounty_program_id);
    log::debug!("[CLI] protocol pubkey {}", protocol.0.to_string());
    let accounts = accounts::Initialize {
        sand_token_mint,
        sand_token_account: sand_token_account.0,
        creator: payer.pubkey(),
        protocol: protocol.0,
        fee_collector,
        collection: nft_collection,
        system_program: system_program::ID,
        token_program: token::ID,
    };
    let data = instruction::Initialize {};

    let ix = anchor_client::solana_sdk::instruction::Instruction {
        program_id: bounty_program_id,
        accounts: accounts.to_account_metas(None),
        data: data.data(),
    };

    let payer_rc = Rc::new(payer);

    let client = anchor_client::Client::new(Cluster::Devnet, payer_rc);
    let program = client.program(bounty_program_id);

    match program.request().instruction(ix).send() {
        Ok(res) => log::info!(
            "Successfully initialized contract {}. {}",
            bounty_program_id.to_string(),
            res
        ),
        Err(err) => log::error!("Failure. cause: {}", err.to_string()),
    };
}

pub fn add_relayer(relayer_address: &Option<String>) {
    let relayer_address = match relayer_address {
        Some(rec) => Pubkey::from_str(rec).unwrap(),
        None => {
            log::error!("Could not find relayer");
            return;
        }
    };
    let payer = load_keypair().unwrap();
    let bounty_program_id = bounty::id();
    let protocol =
        Pubkey::find_program_address(&[utils::BOUNTY_SEED.as_bytes()], &bounty_program_id);
    let relayer = Pubkey::find_program_address(
        &[
            utils::BOUNTY_SEED.as_bytes(),
            relayer_address.to_bytes().as_ref(),
        ],
        &bounty_program_id,
    );

    let accounts = accounts::AddRelayer {
        signer: payer.pubkey(),
        protocol: protocol.0,
        relayer: relayer.0,
        system_program: system_program::ID,
    };

    let data = instruction::AddRelayer {
        relayer_address: relayer_address,
    };

    let add_relayer_ix = anchor_client::solana_sdk::instruction::Instruction {
        program_id: bounty_program_id,
        accounts: accounts.to_account_metas(None),
        data: data.data(),
    };

    let payer_rc = Rc::new(payer);
    let client = anchor_client::Client::new(Cluster::Devnet, payer_rc);
    let program = client.program(bounty_program_id);

    match program.request().instruction(add_relayer_ix).send() {
        Ok(res) => log::info!(
            "Successfully added relayer {}. TxHash={}",
            relayer_address.to_string(),
            res
        ),
        Err(err) => log::error!("Failure. cause: {:?}", err),
    };
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
        Some(Commands::CreateMint { receiver }) => create_mint(receiver),
        Some(Commands::AddRelayer { relayer }) => add_relayer(relayer),

        None => log::error!("Command not found"),
    }
}
