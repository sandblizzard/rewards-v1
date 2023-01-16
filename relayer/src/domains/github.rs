use std::{
    env,
    f32::consts::E,
    future::{self, Future},
    result::Result,
};

use super::{utils::SBError, Domain, DomainHandler, DomainType};
use crate::domains::utils::get_bounty;
use async_trait::async_trait;
use octocrab::*;
pub struct Github {
    pub domain: Domain,
}

#[async_trait]
impl DomainHandler for Github {
    async fn handle(&self) -> Result<(), SBError> {
        match self.domain.bounty_type.as_str() {
            "issue" => self.issues().await,
            _ => Err(SBError::UndefinedBountyType(format!(
                "could not find {} type",
                self.domain.bounty_type.as_str()
            ))),
        }
    }

    fn name(&self) -> String {
        return "github".to_string();
    }
}

impl Github {
    async fn issues(&self) -> Result<(), SBError> {
        log::info!(
            "[relayer] Index github issue for domain={}, repo={} ",
            self.domain.owner,
            self.domain.sub_domain_name
        );
        let github_token = match env::var("GITHUB_TOKEN") {
            Ok(token) => token,
            Err(err) => return Err(SBError::GithubTokenNotSet),
        };

        // initialize github client with personal Access token
        let gh = OctocrabBuilder::new()
            .personal_token(github_token)
            .build()
            .unwrap();
        let mut issues = match gh
            .issues(&self.domain.owner, &self.domain.sub_domain_name)
            .list()
            .state(params::State::Open)
            .send()
            .await
        {
            Ok(val) => val,
            Err(err) => return Err(SBError::FailedToGetIssue(err.to_string())),
        };

        loop {
            for issue in &issues {
                log::info!("[relayer] found issue id={} ", issue.id);
                let issue_body = match issue.body.as_ref() {
                    Some(body) => body,
                    None => {
                        log::warn!(
                            "[relayer] Expected issue body to exist for issue={}, but was empty. Continuing to next issue...",
                            issue.id
                        );
                        continue;
                    }
                };
                // index the bounty information
                let bounty = match get_bounty(&issue.user.id.to_string(), issue_body, &issue.id.0) {
                    Ok(bounty) => bounty,
                    Err(err) => {
                        log::warn!(
                            "issue={}, body={}. Cause={}.  Continuing to next issue...",
                            issue.id,
                            issue_body,
                            err
                        );
                        continue;
                    }
                };
                // try to create the bounty
                bounty.try_create_bounty().unwrap();
            }

            // move to next issue
            issues = match gh
                .get_page::<models::issues::Issue>(&issues.next)
                .await
                .unwrap()
            {
                Some(next_page) => next_page,
                None => break,
            }
        }
        Ok(())
    }
}
