pub mod bounty_proto;
pub mod bounty_sdk;
pub mod domains;
pub mod external;

use anchor_client::solana_sdk::signature::Keypair;

use domains::{
    github::utils::try_fetch_github_indexable_domains,
    utils::{get_key_from_env, SBError},
    *,
};

use std::result::Result;
use tokio::{self};

pub fn load_keypair() -> Result<Keypair, SBError> {
    let key = get_key_from_env("KEY").unwrap();
    let keypair_bytes = key
        .split(",")
        .into_iter()
        .map(|val| {
            return val.parse::<u8>().unwrap();
        })
        .collect::<Vec<u8>>();
    let keypair = Keypair::from_bytes(&keypair_bytes).unwrap();
    Ok(keypair)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    loop {
        // index domains for bounties
        let search_domains = try_fetch_github_indexable_domains().await.unwrap();
        for domain in &search_domains {
            log::info!(
                "[relayer] try index: {} with num repos {:?}",
                domain.name,
                domain.repos.len()
            );
            let domain_type = domain.get_type().await.unwrap();
            match domain_type.handle().await {
                Ok(_) => log::info!(
                    "[relayer] successfully handled domain={}",
                    domain_type.name()
                ),
                Err(err) => log::warn!(
                    "[relayer] failed to handle domain {} with errors: {}",
                    domain_type.name(),
                    err
                ),
            }
        }
    }

    Ok(())
}
