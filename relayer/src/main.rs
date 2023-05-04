pub mod bounty_proto;
pub mod domains;
pub mod external;

use domains::github::utils::try_fetch_github_indexable_domains;

use tokio::{self};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    loop {
        // index domains for bounties
        let search_domains = try_fetch_github_indexable_domains().await.unwrap();
        for domain in &search_domains {
            log::info!(
                "[relayer] try index: {}",
                domain.domain_state.domain.sub_domain
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
        tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
    }
}
