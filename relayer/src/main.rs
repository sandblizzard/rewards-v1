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
    },
    Cluster,
};
use bounty;
use bounty_sdk::*;
use domains::{
    utils::{get_key_from_env, SBError},
    *,
};
pub use jobs::verification;
use jobs::verification::verify_users;
use std::{path::PathBuf, rc::Rc, thread, time};
use std::{result::Result, sync::Arc};
use tokio::{self, sync::Mutex};

/// try_fetch_indexable_domains
///
/// get all domains that are to be indexed
/// FIXME: get the domains from the bounty contract
pub fn try_fetch_indexable_domains() -> Result<Vec<Domain>, SBError> {
    let test_domain = Domain {
        name: "github".to_string(),
        owner: "sandblizzard".to_string(),
        sub_domain_name: "rewards-v1".to_string(),
        bounty_type: "issue".to_string(),
        num_fails: 0,
    };
    let search_domains: Vec<Domain> = [test_domain].to_vec();
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
    let payer = match read_keypair_file("./relayer.json") {
        Ok(payer) => payer,
        Err(err) => return Err(SBError::FailedToLoadKeypair(err.to_string())),
    };
    Ok(payer)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    loop {
        // index domains for bounties
        let search_domains = try_fetch_indexable_domains().unwrap();
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
