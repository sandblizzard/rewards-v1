use std::rc::Rc;

use bounty::state::{domain_data::DomainData, Domain};
use bounty_sdk::utils::{get_key_from_env, SBError};
use futures::future::join_all;
use octocrab::models::issues::Comment;
use spl_associated_token_account::solana_program::pubkey::Pubkey;

use crate::{domains::RelayerDomain, external::get_octocrab_instance};

pub static NUMBER_OF_DAYS_OPEN: u64 = 60 * 60 * 24 * 10;

/// try_fetch_indexable_domains
///
/// get all domains that are to be indexed
/// FIXME: get the domains from the bounty contract
pub async fn try_fetch_github_indexable_domains() -> Result<Vec<RelayerDomain>, SBError> {
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
            Err(err) => {
                log::error!(
                    "could not get repos for {}. Cause: {:?}",
                    domain.account.login,
                    err.to_string()
                );
                Vec::new()
            }
        };
        let domains = repos.into_iter().map(|repo| RelayerDomain {
            domain_state: Domain {
                bump: 0,
                active: true,
                data: DomainData {
                    organization: domain.account.login.to_string(),
                    domain_type: "issue".to_string(),
                    platform: "github".to_string(),
                    team: repo.name,
                },

                owner: Pubkey::new_from_array([0; 32]),
            },
            access_token_url: domain
                .access_tokens_url
                .clone()
                .unwrap_or_else(|| "".to_string()),
        });
        domains.collect::<Vec<RelayerDomain>>()
    }))
    .await;

    Ok(search_domains.into_iter().flatten().collect())
}

/// comment_contains_signing_link
///
/// checks if a comment contains the sandblizzard domain
pub fn comment_contains_signing_link(comment: &Comment) -> Result<bool, SBError> {
    let comment_body = match &comment.body {
        Some(body) => body,
        None => return Ok(false),
    };
    let sb_bounty_domain = get_key_from_env("SANDBLIZZARD_URL")?;
    Ok(comment_body.contains(&sb_bounty_domain))
}

/// contains_bounty_status
///
/// checks if a comment contains the given bounty status
pub fn contains_bounty_status(comment: &Comment, bounty_status: &str) -> bool {
    let comment_body = match &comment.body {
        Some(body) => body,
        None => return false,
    };
    bounty_status.contains(comment_body)
}

/// create_bounty_status_text

pub fn create_bounty_status_text(
    bounty: &bounty::state::Bounty,
    sig: Option<&str>,
) -> Result<String, SBError> {
    let mut status = format!("Bounty status: **{:?}** ", bounty.state);
    if sig.is_some() {
        status = format!(
            "{} \n
        Signature: {}
        ",
            status,
            sig.unwrap()
        )
    }
    Ok(status)
}
