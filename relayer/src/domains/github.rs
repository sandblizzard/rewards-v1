use std::{
    result::Result,
    thread,
};

use super::{
    utils::{get_key_from_env, SBError},
    Domain, DomainHandler,
};
use crate::{
    bounty_proto::{get_solvers, BountyProto},
    bounty_sdk::{BountySdk},
    domains::utils::get_unix_time,
    external::{get_connection, is_relayer_login},
};

use async_trait::async_trait;
use bounty;
use futures::future::join_all;
use log::{info};
use octocrab::{
    models::{
        issues::{Comment, Issue},
    },
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
    /// Create new Github interface
    pub async fn new(domain: &Domain) -> Result<Github, SBError> {
        let github_client = get_connection(&domain.access_token_url).await?;
        Ok(Github {
            domain: domain.clone(),
            gh: Some(github_client),
        })
    }

    /// issues
    ///
    /// Handles the github issues
    pub async fn issues(&self) -> Result<(), SBError> {
        log::info!(
            "[relayer] Index github issue for domain={}",
            self.domain.owner,
        );

        let issues: Vec<Vec<Issue>> =
            join_all(self.domain.repos.clone().iter().map(|repo| async move {
                let issue_handler = self
                    .gh
                    .as_ref()
                    .unwrap()
                    .issues(&self.domain.owner, &repo.name);
                let mut issues = match issue_handler
                    .list()
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

                return issues.take_items();
            }))
            .await;

        log::info!(
            "[relayer] {} issues for {} ",
            issues.len(),
            self.domain.name,
        );

        let issues_flat: Vec<SBIssue> = issues
            .iter()
            .flatten()
            .filter(|&issue| {
                issue
                    .created_at
                    .timestamp()
                    .ge(&(get_unix_time(60 * 60 * 24 * 2) as i64))
            })
            .map(|issue| {
                let repo = issue
                    .repository_url
                    .path()
                    .split("/")
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
                info!("Issue {}", issue.url);
                issue.handle().await
            });
            handles.push(handle)
        }

        for handle in handles {
            match handle.join() {
                Ok(res) => {
                    res.await;
                }
                Err(err) => {
                    log::error!("Faild to join {:?}", err);
                    return Err(SBError::IssueNotClosed);
                }
            }
        }

        Ok(())
    }
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

#[derive(Debug)]
pub struct SBIssue {
    id: u64,
    creator: String,
    access_token_url: String,
    owner: String,
    repo: String,
    number: i64,
    url: String,
    state: String,
    body: Option<String>,
    closed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl SBIssue {
    async fn get_bounty_from_issue(&self) -> Result<BountyProto, SBError> {
        let issue_body = match self.body.as_ref() {
            Some(body) => body,
            None => {
                return Err(SBError::FailedToFindBounty(
                    "No body found on issue".to_string(),
                ))
            }
        };
        // index the bounty information
        let bounty = match BountyProto::new_bounty_proto(&self.creator, &issue_body, &self.id).await
        {
            Ok(bounty) => bounty,
            Err(err) => return Err(SBError::FailedToFindBounty(err.to_string())),
        };

        Ok(bounty)
    }

    pub async fn try_post_bounty_status(
        &self,
        status: &str,
        comments: &Vec<&Comment>,
    ) -> Result<(), SBError> {
        if comments
            .iter()
            .any(|comment| contains_bounty_status(comment, &status))
        {
            return Ok(());
        } else {
            self.post_bounty_status(status).await
        }
    }

    pub async fn post_bounty_status(&self, status: &str) -> Result<(), SBError> {
        let gh = get_connection(&self.access_token_url).await?;
        return match gh
            .issues(&self.owner, &self.repo)
            .create_comment(self.number as u64, status)
            .await
        {
            Ok(comment) => {
                log::info!(
                    "[relayer] successfully created comment {}",
                    comment.issue_url.unwrap()
                );
                Ok(())
            }
            Err(err) => Err(SBError::FailedToComment(
                "post_bounty_status".to_string(),
                err.to_string(),
            )),
        };
    }

    /// try_get_sandblizzard confirmation
    ///
    /// when an issue is closed and the app has completed the
    /// bounty then a message is emitted

    /// try_get_closing_comment
    ///
    /// will try to get the comments associated with the closing of
    /// an issue
    async fn try_get_closing_comment<'a>(
        &self,
        comments: &Vec<Comment>,
    ) -> Result<String, SBError> {
        // get comments on issue

        let issue_closed_at = match self.closed_at {
            Some(timestamp) => timestamp,
            None => return Err(SBError::IssueNotClosed),
        };
        // filter comments at closing
        let comments: Vec<&Comment> = comments
            .iter()
            .filter(|comment| comment.created_at.eq(&issue_closed_at))
            .collect();

        // take first closed comment
        let first_close_issue_comment = match comments.first() {
            Some(comment) => comment,
            None => {
                return Err(SBError::CommentNotFound(
                    "closed issue".to_string(),
                    "".to_string(),
                ))
            }
        };

        let comment_body = match first_close_issue_comment.body.as_ref() {
            Some(comment) => comment,
            None => {
                return Err(SBError::CommentNotFound(
                    "closed body issue".to_string(),
                    "Comment body not found".to_string(),
                ))
            }
        };
        Ok(comment_body.clone())
    }

    /// create_signing_link
    ///
    /// creates a link with enough query params to create a `create_bounty` tx
    async fn post_signing_link(
        &self,
        bounty_amount: &f64,
        mint: &str,
        token_name: &str,
    ) -> Result<(), SBError> {
        log::info!(
            "[relayer] try to create signing link for issue number: {} ",
            self.number
        );
        let gh = get_connection(&self.access_token_url).await?;
        return match gh
            .issues(&self.owner, &self.repo)
            .create_comment(
                self.number.try_into().unwrap(),
                self.get_signing_link(bounty_amount, mint, token_name)?,
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
            Err(err) => Err(SBError::FailedToComment(
                "create_signing_link".to_string(),
                err.to_string(),
            )),
        };
    }

    /// get signing link
    ///
    /// generates a signing link in order to generate a tx
    pub fn get_signing_link(
        &self,
        bounty_amount: &f64,
        mint: &str,
        token_name: &str,
    ) -> Result<String, SBError> {
        let sb_bounty_domain = get_key_from_env("SANDBLIZZARD_URL")?;

        let referrer = format!(
            "https://github.com/{}/{}/issues/{}",
            self.owner, self.repo, self.number
        );
        Ok(format!(
            "Create bounty by signing: [Transaction]({}/create_bounty?referrer={}&domain={}&subDomain={}&id={}&bountyAmount={}&mint={}&token={})",
            sb_bounty_domain,referrer, self.owner, self.repo, self.id,bounty_amount,mint,token_name
        ))
    }

    async fn open_issue(&self) -> Result<(), SBError> {
        log::info!("[issue] Open issue for {}", self.number);
        let bounty = BountySdk::new()?.get_bounty(&self.owner, &self.repo, &self.id);

        let gh = get_connection(&self.access_token_url).await?;
        let comments: Vec<Comment> = gh
            .issues(&self.owner, &self.repo)
            .list_comments(self.number as u64)
            .per_page(150)
            .send()
            .await
            .map_err(|err| SBError::CommentsNotFound("open issues".to_string(), err.to_string()))?
            .take_items();
        let mut relayer_comments_iter = comments
            .iter()
            .filter(|comment| is_relayer_login(&comment.user.login).unwrap());

        match bounty {
            Ok(bounty) => {
                log::info!("[Issue] bounty exists {}", self.id);
                let status = create_bounty_status_text(&bounty, None)?;
                self.try_post_bounty_status(&status, &relayer_comments_iter.collect())
                    .await?;
            }
            Err(err) => {
                // if issue is open, but bounty does not exist -> check if bounty is proposed
                log::info!("issue {} not created. Cause {}", self.id, err.to_string());
                // get bounty if proposed in issue
                let bounty_proposed_in_issue = self.get_bounty_from_issue().await?;

                // Check the status of the bounty
                // -> If there is no signing link -> look for bounty -> post signing link
                // get the top 150 comments on the issue

                let has_posted_signing_link = &relayer_comments_iter
                    .any(|comment| comment_contains_signing_link(&comment).unwrap());
                log::info!(
                    "Has posted signing link for {}: {}",
                    self.url,
                    has_posted_signing_link
                );
                // bounty don't exist
                if !(*has_posted_signing_link) {
                    // if bounty is new then generate signing link
                    self.post_signing_link(
                        &bounty_proposed_in_issue.amount.unwrap(),
                        &bounty_proposed_in_issue.token_mint.unwrap(),
                        &bounty_proposed_in_issue.token_name.unwrap(),
                    )
                    .await?;
                }
                log::info!("issues: bounty for issue={} does not exists and signing link has been posted={} ",self.id,has_posted_signing_link);
            }
        };
        Ok(())
    }

    pub async fn close_issue(&self) -> Result<(), SBError> {
        log::info!("[issue] Close issue for {}", self.number);
        let bounty = BountySdk::new()?.get_bounty(&self.owner, &self.repo, &self.id)?;

        // get the top 150 comments on the issue
        let page_comments = get_connection(&self.access_token_url)
            .await?
            .issues(&self.owner, &self.repo)
            .list_comments(self.number as u64)
            .per_page(150)
            .send()
            .await
            .map_err(|err| SBError::CommentsNotFound("closed issues".to_string(), err.to_string()))?
            .take_items();

        // try to get the comment body. If no closing comment -> return

        let solvers = get_solvers(&self.creator.to_string()).await?;

        let (bounty, sig) = BountySdk::new()?.complete_bounty(
            &self.owner,
            &self.repo,
            &self.id,
            &solvers,
            &bounty.mint,
        )?;

        // try post bounty statys
        let relayer_comments_iter = page_comments
            .iter()
            .filter(|comment| is_relayer_login(&comment.user.login).unwrap());
        let status = create_bounty_status_text(&bounty, Some(&sig))?;
        self.try_post_bounty_status(&status, &relayer_comments_iter.collect())
            .await?;
        Ok(())
    }

    pub async fn handle(&self) -> Result<(), SBError> {
        if self.state.eq("open") {
            // -> If open -> try to complete bounty
            match self.open_issue().await {
                Ok(res) => res,
                Err(err) => {
                    log::warn!(
                        "Could not handle open issue for {}. Cause {}",
                        self.url,
                        err
                    );
                }
            };
        } else {
            // -> If closed -> try to complete bounty
            match self.close_issue().await {
                Ok(res) => res,
                Err(err) => {
                    log::warn!(
                        "Could not handle closed issue for {}. Cause {}",
                        self.url,
                        err
                    );
                }
            }
        }

        Ok(())
    }
}
