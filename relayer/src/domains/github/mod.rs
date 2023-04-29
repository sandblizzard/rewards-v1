use std::{result::Result, thread};
pub mod issues;
pub mod utils;
use super::{
    utils::{SBError},
    Domain, DomainHandler,
};
use crate::{
    domains::utils::get_unix_time,
    external::{get_connection},
};
use issues::SBIssue;

use async_trait::async_trait;
use futures::future::join_all;
use log::{error, info};
use octocrab::{
    models::issues::{Issue},
    *,
};

pub struct Github {
    pub domain: Domain,
    pub gh: Option<Octocrab>,
}

#[async_trait]
impl DomainHandler for Github {
    async fn handle(&self) -> Result<(), SBError> {
        match self.domain.bounty_type.as_str() {
            "issue" => self.issues().await,
            "pull_request" => self.pull_requests().await,
            _ => Err(SBError::UndefinedBountyType(format!(
                "could not find {} type",
                self.domain.bounty_type.as_str()
            ))),
        }
    }

    fn name(&self) -> String {
        "github".to_string()
    }
}

impl Github {
    /// Create new Github interface
    pub async fn new(domain: &Domain) -> Result<Github, SBError> {
        let github_client = get_connection(&domain.access_token_url).await?;
        Ok(Github {
            domain: domain.clone(),
            gh: Some(github_client),
        })
    }

    /// handle pull request bounties
    pub async fn pull_requests(&self) -> Result<(), SBError> {
        Err(SBError::NotImplemented(String::from("pull_request")))
    }

    /// issues
    ///
    /// Handles github issues
    pub async fn issues(&self) -> Result<(), SBError> {
        log::info!(
            "[relayer] Index github issue for domain={}",
            self.domain.owner,
        );

        // get all issues for a given domain over different repositories
        let issues: Vec<Vec<Issue>> =
            join_all(self.domain.repos.clone().iter().map(|repo| async move {
                let issue_handler = self
                    .gh
                    .as_ref()
                    .unwrap()
                    .issues(&self.domain.owner, &repo.name);

                // get top 100 issues
                let mut issues = match issue_handler
                    .list()
                    .sort(params::issues::Sort::Created)
                    .state(params::State::All)
                    .per_page(100)
                    .send()
                    .await
                {
                    Ok(issues) => issues,
                    Err(_) => {
                        log::warn!("[relayer] Could not get issues for {}", repo.name);
                        return Vec::new();
                    }
                };

                issues.take_items()
            }))
            .await;

        log::info!(
            "[relayer] {} issues for name: {}, owner: {}",
            issues.len(),
            self.domain.name,
            self.domain.owner,
        );

        let issues_flat: Vec<SBIssue> = issues
            .iter()
            .flatten()
            .filter(|&issue| {
                issue
                    .created_at
                    .timestamp()
                    .ge(&(get_unix_time(60 * 60 * 24 * 2) as i64)) // only consider <2d old issues
            })
            .map(|issue| {
                let repo = issue
                    .repository_url
                    .path()
                    .split('/')
                    .collect::<Vec<&str>>();
                return SBIssue {
                    id: issue.id.0,
                    creator: issue.user.id.0.to_string(),
                    access_token_url: self.domain.access_token_url.clone(),
                    owner: self.domain.owner.clone(),
                    repo: repo.last().unwrap_or(&"").to_string(),
                    number: issue.number,
                    url: issue.url.to_string(),
                    state: issue.state.to_string(),
                    body: issue.body.clone(),
                    closed_at: issue.closed_at,
                };
            })
            .collect();

        let mut handles = vec![];
        for issue in issues_flat {
            // get Status of Issue
            // 1. Open - try create bounty
            // 2. Closed -
            //  - pay out bounty if mentioned users
            //  - close bounty if no one mentioned
            let handle = thread::spawn(|| async move {
                info!("[Issues] handle issue {}", issue.url);
                issue.handle().await
            });
            handles.push(handle)
        }

        for handle in handles {
            match handle.join() {
                Ok(res) => match res.await {
                    Ok(res) => {
                        info!("[Issues] Successfully handled issue {}", res)
                    }
                    Err(err) => {
                        error!("[Issues] Failed to handle issue! Cause {}", err)
                    }
                },
                Err(err) => {
                    log::error!("Faild to join {:?}", err);
                    return Err(SBError::IssueNotClosed);
                }
            }
        }

        Ok(())
    }
}
