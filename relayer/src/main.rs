pub mod bounty_proto;
pub mod bounty_sdk;
pub mod domains;
pub mod external;


use anchor_client::{
    solana_sdk::{
        signature::{Keypair},
    },
};
use external::get_octocrab_instance;
use futures::future::join_all;



use domains::{
    utils::{get_key_from_env, SBError},
    *,
};



use std::{rc::Rc};
use std::{result::Result};
use tokio::{self};

/// try_fetch_indexable_domains
///
/// get all domains that are to be indexed
/// FIXME: get the domains from the bounty contract
pub async fn try_fetch_indexable_domains() -> Result<Vec<Domain>, SBError> {
    let gh = Rc::new(get_octocrab_instance()?);
    let domains = match gh.apps().installations().send().await {
        Ok(res) => res,
        Err(err) => {
            return Err(SBError::FailedOctocrabRequest(
                "try_fetch_indexable_domains".to_string(),
                err.to_string(),
            ))
        }
    };
    let search_domains = join_all(domains.into_iter().map(|domain| async move {
        let repos = match get_octocrab_instance()
            .unwrap()
            .installation(domain.id)
            .orgs(&domain.account.login)
            .list_repos()
            .send()
            .await
        {
            Ok(mut repos) => repos.take_items(),
            Err(_) => Vec::new(),
        };
        return Domain {
            name: "github".to_string(),
            owner: domain.account.login.clone(),
            repos: repos,
            access_token_url: domain.access_tokens_url.unwrap_or("".to_string()),
            bounty_type: "issue".to_string(),
            num_fails: 0,
        };
    }))
    .await;

    Ok(search_domains)
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
