use std::rc::Rc;

use futures::future::join_all;
use octocrab::models::issues::Comment;

use crate::{
    domains::{
        utils::{get_key_from_env, SBError},
        Domain,
    },
    external::get_octocrab_instance,
};

/// try_fetch_indexable_domains
///
/// get all domains that are to be indexed
/// FIXME: get the domains from the bounty contract
pub async fn try_fetch_github_indexable_domains() -> Result<Vec<Domain>, SBError> {
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
        return vec![Domain {
            name: "github".to_string(),
            owner: domain.account.login.clone(),
            repos: repos,
            access_token_url: domain.access_tokens_url.unwrap_or("".to_string()),
            bounty_type: "issue".to_string(),
            num_fails: 0,
        }];
    }))
    .await;

    Ok(search_domains)
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
    let mut status = format!("Bounty status: **{}** ", bounty.state.to_uppercase());
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
