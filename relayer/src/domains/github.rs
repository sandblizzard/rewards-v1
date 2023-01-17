use std::{
    result::Result,
};

use super::{
    utils::{get_key_from_env, SBError},
    Domain, DomainHandler,
};
use crate::domains::bounty::{get_bounty, get_solvers};
use async_trait::async_trait;
use octocrab::{
    models::{issues::Comment, IssueState},
    params::apps::CreateInstallationAccessToken,
    *,
};
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

/// get_connection establish a connection with github
pub async fn get_connection() -> Result<Octocrab, SBError> {
    let github_key = get_key_from_env("GITHUB_KEY")?;
    let github_id = get_key_from_env("GITHUB_ID")?;

    let app_id = github_id.parse::<u64>().unwrap().into();
    let key = jsonwebtoken::EncodingKey::from_rsa_pem(github_key.as_bytes()).unwrap();
    let token = octocrab::auth::create_jwt(app_id, &key).unwrap();
    let gh = Octocrab::builder().personal_token(token).build().unwrap();
    let installations = gh.apps().installations().send().await.unwrap().take_items();
    let access_token = CreateInstallationAccessToken::default();

    let access: models::InstallationToken = gh
        .post(
            installations[0].access_tokens_url.as_ref().unwrap(),
            Some(&access_token),
        )
        .await
        .unwrap();
    Ok(octocrab::OctocrabBuilder::new()
        .personal_token(access.token)
        .build()
        .unwrap())
}

impl Github {
    async fn create_signing_link(
        &self,
        gh: &Octocrab,
        issue_number: &u64,
        issue_id: &u64,
    ) -> Result<(), SBError> {
        log::info!(
            "[relayer] try to create signing link for issue_id: {} ",
            issue_id
        );
        return match gh
            .issues(&self.domain.owner, &self.domain.sub_domain_name)
            .create_comment(
                *issue_number,
                format!(
                    "Create bounty by signing: [Transaction](https://bounty.sandblizzard/new?owner={},repo={},id={})",
                    self.domain.owner, self.domain.sub_domain_name, issue_id
                ),
            )
            .await
        {
            Ok(comment) => {
                log::info!(
                    "[relayer] successfully created comment {}",
                    comment.issue_url.unwrap()
                );
                Ok(())
            }
            Err(err) => Err(SBError::CouldNotGenerateSigningLink(err.to_string())),
        };
    }

    async fn issues(&self) -> Result<(), SBError> {
        log::info!(
            "[relayer] Index github issue for domain={}, repo={} ",
            self.domain.owner,
            self.domain.sub_domain_name
        );

        let gh = get_connection().await?;
        let issues_cursor = gh.issues(&self.domain.owner, &self.domain.sub_domain_name);
        let mut issues = match issues_cursor.list().state(params::State::All).send().await {
            Ok(val) => val,
            Err(err) => return Err(SBError::FailedToGetIssue(err.to_string())),
        };

        loop {
            for issue in &issues {
                // get Status of Issue
                // 1. Open - try create bounty
                // 2. Closed -
                //  - pay out bounty if mentioned users
                //  - close bounty if no one mentioned

                if issue.state == IssueState::Open {
                    log::info!(
                        "[relayer] found issue id={}, isOpen= {}",
                        issue.id,
                        issue.state == IssueState::Open,
                    );
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
                    let bounty =
                        match get_bounty(&issue.user.id.to_string(), issue_body, &issue.id.0) {
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
                    match bounty.try_create_bounty() {
                        Ok(res) => res,
                        Err(res) => {
                            if res == SBError::BountyExists {
                                log::info!("Bounty already exists. Nothing more todo ");
                                continue;
                            }
                        }
                    }

                    // if bounty is new then generate signing link
                    self.create_signing_link(&gh, &issue.number, &issue.id.0)
                        .await?;
                } else {
                    // FIXME: Clean up code
                    // Issue is closed -> try to complete the bounty
                    let issue_closed_at = issue.closed_at.unwrap();

                    // get comments on issue
                    let page_comments = issues_cursor
                        .list_comments(issue.number)
                        .per_page(150)
                        .send()
                        .await
                        .unwrap()
                        .take_items();

                    // filter comments at closing
                    let comments: Vec<&Comment> = page_comments
                        .iter()
                        .filter(|comment| comment.created_at.eq(&issue_closed_at))
                        .collect();
                    // take first closed comment
                    let first_close_issue_comment = comments.first().unwrap();
                    let bounty = get_solvers(
                        &issue.user.id.to_string(),
                        &first_close_issue_comment.body.as_ref().unwrap(),
                        &issue.id,
                    )
                    .unwrap();

                    bounty.try_complete_bounty().unwrap()
                }
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
