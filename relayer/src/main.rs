pub mod domains;
pub mod external;
pub mod jobs;

use std::{thread, time};

use anchor_client::anchor_lang::Result;
use domains::*;
pub use jobs::verification;
use log;

use tokio;

/// try_fetch_indexable_domains is supposed to get all the
/// domains stored on chain and return it as a list
pub fn try_fetch_indexable_domains() -> Result<Vec<Domain>> {
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

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    loop {
        let search_domains = try_fetch_indexable_domains().unwrap();
        for domain in &search_domains {
            log::info!("[relayer] try index: {}", domain.name);
            let domain_type = domain.get_type().unwrap();
            match domain_type.handle().await {
                Ok(_) => log::info!(
                    "[relayer] successfully handled domain={}",
                    domain_type.name()
                ),
                Err(err) => log::warn!(
                    "[relayer] failed to handle domain {} with error={}",
                    domain_type.name(),
                    err
                ),
            }
        }

        // sleep for 5s after each loop
        thread::sleep(time::Duration::from_secs(5));
    }
}
