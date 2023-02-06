pub mod bounty_proto;
pub mod bounty_sdk;
pub mod domains;
pub mod external;
pub mod jobs;
use actix_files::*;
use actix_web::{rt::spawn, *};
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        signature::{read_keypair, read_keypair_file, Keypair},
        signer::Signer,
    },
    Cluster,
};
use bounty;
use bounty_sdk::*;
use domains::{
    utils::{get_key_from_env, SBError},
    *,
};
use ed25519_dalek;
pub use jobs::verification;
use jobs::verification::verify_users;
use log::info;
use spl_associated_token_account::solana_program::example_mocks::solana_sdk;
use std::{path::PathBuf, rc::Rc, thread, time};
use std::{result::Result, sync::Arc};
use tokio::{self, sync::Mutex};

/// try_fetch_indexable_domains
///
/// get all domains that are to be indexed
/// FIXME: get the domains from the bounty contract
pub async fn try_fetch_indexable_domains() -> Result<Vec<Domain>, SBError> {
    let gh = get_octocrab_instance()?;
    let domains = match gh.apps().installations().send().await {
        Ok(res) => res,
        Err(err) => {
            return Err(SBError::FailedOctocrabRequest(
                "try_fetch_indexable_domains".to_string(),
                err.to_string(),
            ))
        }
    };
    let search_domains: Vec<Domain> = domains
        .into_iter()
        .map(|domain| {
            info!("domain: {:?}", domain);

            return Domain {
                name: "github".to_string(),
                owner: domain.account.login,
                access_token_url: domain.access_tokens_url.unwrap_or("".to_string()),
                bounty_type: "issue".to_string(),
                num_fails: 0,
            };
        })
        .collect::<Vec<Domain>>();

    Ok(search_domains)
}

/// sign_create_bounty
///
/// returns a web page for signing a create_bounty tx
#[get("/create_bounty")]
async fn serve_static() -> actix_web::Result<NamedFile> {
    let path: PathBuf = "./dist/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

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
        let search_domains = try_fetch_indexable_domains().await.unwrap();
        for domain in &search_domains {
            log::info!("[relayer] try index: {}", domain.name);
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

        // Verify users based on verification file
        match verify_users().await {
            Ok(_) => (),
            Err(err) => {
                log::warn!("[relayer] failed to verify users with error={}", err)
            }
        };

        // sleep for 5s after each loop
        thread::sleep(time::Duration::from_secs(5));
    }

    Ok(())
}
