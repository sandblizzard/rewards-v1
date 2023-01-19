use bounty::{accounts, instruction, utils, Initialize};
use std::{rc::Rc, str::FromStr, sync::Arc};

use anchor_client::{
    anchor_lang::{solana_program::pubkey, system_program, InstructionData, ToAccountMetas},
    solana_client::blockhash_query,
    solana_sdk::{
        pubkey::Pubkey,
        signature::read_keypair_file,
        signer::{self, Signer},
    },
    *,
};
use clap::Parser;
use home;
use log;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// program_id of program
    #[clap(short, long, value_parser)]
    program_id: String,
}

fn main() {
    let env = env_logger::init();
    let args = Args::parse();

    // get keypair from config.id

    let fee_collector = Pubkey::from_str("CNY467c6XURCPjiXiKRLCvxdRf3bpunagYTJpr685gPv").unwrap();

    let nft_collection = Pubkey::from_str("BXKro6nDX9y86rtGn6uh6K1rZUqENzsUHP6gAbdJj1NS").unwrap();
    let program_id = Pubkey::from_str(&args.program_id).unwrap();
    log::debug!("[CLI] program-id: {}", program_id.to_string());
    let keypair_location = format!(
        "{}/.config/solana/id.json",
        home::home_dir().unwrap().to_str().unwrap()
    );
    log::debug!("[CLI] Try keypair location {} ", keypair_location);
    let payer = read_keypair_file(keypair_location).unwrap();

    let protocol = Pubkey::find_program_address(&[utils::BOUNTY_SEED.as_bytes()], &program_id);
    log::debug!("[CLI] protocol pubkey {}", protocol.0.to_string());
    let accounts = accounts::Initialize {
        creator: payer.pubkey(),
        protocol: protocol.0,
        fee_collector,
        collection: nft_collection,
        system_program: system_program::ID,
    };
    let data = instruction::Initialize {};

    let ix = anchor_client::solana_sdk::instruction::Instruction {
        program_id,
        accounts: accounts.to_account_metas(None),
        data: data.data(),
    };

    let payer_rc = Rc::new(payer);

    let client = anchor_client::Client::new(Cluster::Devnet, payer_rc);
    let program = client.program(program_id);

    match program.request().instruction(ix).send() {
        Ok(res) => log::info!("Success {}", res),
        Err(err) => log::error!("Failure. cause: {}", err.to_string()),
    };
}
