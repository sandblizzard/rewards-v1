use std::{
    fmt::format,
    rc::{self, Rc},
    result::Result,
    sync::Arc,
    thread, time,
};

use super::{
    utils::{get_key_from_env, SBError},
    Domain, DomainHandler,
};
use crate::{
    bounty_proto::{get_solvers, BountyProto},
    bounty_sdk::{self, BountySdk},
    domains::utils::get_unix_time,
    external::status_manager,
};
use anchor_client::{Client, Program};
use async_trait::async_trait;
use bounty;
use log::{debug, info};
use octocrab::{
    models::{
        issues::{Comment, Issue},
        InstallationId, IssueId, IssueState,
    },
    params::apps::CreateInstallationAccessToken,
    *,
};
use tokio::sync::Mutex;
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

pub fn is_relayer_login(login: &str) -> Result<bool, SBError> {
    let app_login = get_key_from_env("GITHUB_APP_LOGIN")?;
    Ok(login.eq(&app_login))
}

pub fn get_octocrab_instance() -> Result<Octocrab, SBError> {
    let github_key = get_key_from_env("GITHUB_KEY")?;
    let github_id = get_key_from_env("GITHUB_ID")?;
    let app_id = github_id.parse::<u64>().unwrap().into();
    let key = jsonwebtoken::EncodingKey::from_rsa_pem(github_key.as_bytes()).unwrap();
    match Octocrab::builder().app(app_id, key).build() {
        Ok(gh) => Ok(gh),
        Err(err) => {
            return Err(SBError::FailedOctocrabRequest(
                "get_octocrab_instance".to_string(),
                err.to_string(),
            ))
        }
    }
}

/// get_connection establish a connection with github
pub async fn get_connection(access_token_url: &str) -> Result<Octocrab, SBError> {
    let github_key = get_key_from_env("GITHUB_KEY")?;
    let github_id = get_key_from_env("GITHUB_ID")?;

    let app_id = github_id.parse::<u64>().unwrap().into();
    let key = jsonwebtoken::EncodingKey::from_rsa_pem(github_key.as_bytes()).unwrap();
    let token = octocrab::auth::create_jwt(app_id, &key).unwrap();
    let gh = Octocrab::builder().personal_token(token).build().unwrap();

    let access_token = CreateInstallationAccessToken::default();

    let access: models::InstallationToken = gh
        .post(access_token_url, Some(&access_token))
        .await
        .unwrap();
    Ok(octocrab::OctocrabBuilder::new()
        .personal_token(access.token)
        .build()
        .unwrap())
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

    async fn get_bounty_from_issue(&self, issue: &Issue) -> Result<BountyProto, SBError> {
        let issue_body = match issue.body.as_ref() {
            Some(body) => body,
            None => {
                return Err(SBError::FailedToFindBounty(
                    "No body found on issue".to_string(),
                ))
            }
        };

        // index the bounty information
        let bounty = match BountyProto::new_bounty_proto(
            &issue.user.id.to_string(),
            issue_body,
            &issue.id.0,
        )
        .await
        {
            Ok(bounty) => bounty,
            Err(err) => return Err(SBError::FailedToFindBounty(err.to_string())),
        };

        Ok(bounty)
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
        issue: &Issue,
        comments: &Vec<Comment>,
    ) -> Result<String, SBError> {
        // get comments on issue

        let issue_closed_at = match issue.closed_at {
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

    /// comment_contains_signing_link
    ///
    /// checks if a comment contains the sandblizzard domain
    pub fn comment_contains_signing_link(&self, comment: &Comment) -> Result<bool, SBError> {
        let comment_body = match &comment.body {
            Some(body) => body,
            None => return Ok(false),
        };
        let sb_bounty_domain = get_key_from_env("SANDBLIZZARD_URL")?;
        Ok(comment_body.contains(&sb_bounty_domain))
    }

    /// get signing link
    ///
    /// generates a signing link in order to generate a tx
    pub fn get_signing_link(
        &self,
        issue_id: &u64,
        issue_number: &u64,
        bounty_amount: &f64,
        mint: &str,
        token_name: &str,
    ) -> Result<String, SBError> {
        let sb_bounty_domain = get_key_from_env("SANDBLIZZARD_URL")?;

        let referrer = format!(
            "https://github.com/{}/{}/issues/{}",
            self.domain.owner, self.domain.sub_domain_name, issue_number
        );
        Ok(format!(
            "Create bounty by signing: [Transaction]({}/create_bounty?referrer={}&domain={}&subDomain={}&id={}&bountyAmount={}&mint={}&token={})",
            sb_bounty_domain,referrer, self.domain.owner, self.domain.sub_domain_name, issue_id,bounty_amount,mint,token_name
        ))
    }

    /// contains_bounty_status
    ///
    /// checks if a comment contains the given bounty status
    pub fn contains_bounty_status(&self, comment: &Comment, bounty_status: &str) -> bool {
        let comment_body = match &comment.body {
            Some(body) => body,
            None => return false,
        };
        bounty_status.contains(comment_body)
    }

    /// create_bounty_status_text
    pub fn create_bounty_status_text(
        &self,
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

    pub async fn try_post_bounty_status(
        &self,
        issue_number: &u64,
        status: &str,
        comments: &Vec<&Comment>,
    ) -> Result<(), SBError> {
        if comments
            .iter()
            .any(|comment| self.contains_bounty_status(comment, &status))
        {
            return Ok(());
        } else {
            self.post_bounty_status(issue_number, status).await
        }
    }

    pub async fn post_bounty_status(
        &self,
        issue_number: &u64,
        status: &str,
    ) -> Result<(), SBError> {
        return match self
            .gh
            .as_ref()
            .unwrap()
            .issues(&self.domain.owner, &self.domain.sub_domain_name)
            .create_comment((*issue_number).try_into().unwrap(), status)
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

    /// create_signing_link
    ///
    /// creates a link with enough query params to create a `create_bounty` tx
    async fn post_signing_link(
        &self,
        gh: &Octocrab,
        issue_number: &u64,
        issue_id: &u64,
        bounty_amount: &f64,
        mint: &str,
        token_name: &str,
    ) -> Result<(), SBError> {
        log::info!(
            "[relayer] try to create signing link for issue number: {} ",
            issue_number
        );
        return match gh
            .issues(&self.domain.owner, &self.domain.sub_domain_name)
            .create_comment(
                (*issue_number).try_into().unwrap(),
                self.get_signing_link(issue_id, issue_number, bounty_amount, mint, token_name)?,
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

    async fn handle_open_issue(&self, issue: &Issue) -> Result<(), SBError> {
        log::info!(
            "[relayer] found issue id={}, isOpen= {}",
            issue.id,
            issue.state.eq("open"),
        );

        let bounty = BountySdk::new()?.get_bounty(
            &self.domain.owner,
            &self.domain.sub_domain_name,
            &issue.id.0,
        );

        let comments: Vec<Comment> = self
            .gh
            .as_ref()
            .unwrap()
            .issues(&self.domain.owner, &self.domain.sub_domain_name)
            .list_comments(issue.number as u64)
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
                let status = self.create_bounty_status_text(&bounty, None)?;
                self.try_post_bounty_status(
                    &(issue.number as u64),
                    &status,
                    &relayer_comments_iter.collect(),
                )
                .await?;
            }
            Err(err) => {
                // if issue is open, but bounty does not exist -> check if bounty is proposed
                log::info!(
                    "issue {} not created. Cause {}",
                    issue.id.0,
                    err.to_string()
                );
                // get bounty if proposed in issue
                let bounty_proposed_in_issue = self.get_bounty_from_issue(&issue.clone()).await?;

                // Check the status of the bounty
                // -> If there is no signing link -> look for bounty -> post signing link
                // get the top 150 comments on the issue

                let has_posted_signing_link = &relayer_comments_iter
                    .any(|comment| self.comment_contains_signing_link(&comment).unwrap());
                log::info!(
                    "Has posted signing link for {}: {}",
                    issue.url,
                    has_posted_signing_link
                );
                // bounty don't exist
                if !(*has_posted_signing_link) {
                    // if bounty is new then generate signing link
                    self.post_signing_link(
                        &self.gh.as_ref().unwrap(),
                        &(issue.number as u64),
                        &issue.id,
                        &bounty_proposed_in_issue.amount.unwrap(),
                        &bounty_proposed_in_issue.token_mint.unwrap(),
                        &bounty_proposed_in_issue.token_name.unwrap(),
                    )
                    .await?;
                }
                log::info!("issues: bounty for issue={} does not exists and signing link has been posted={} ",issue.id.0,has_posted_signing_link);
            }
        };
        Ok(())
    }

    pub async fn handle_closed_issue(&self, issue: &Issue) -> Result<(), SBError> {
        log::info!(
            "Issues: issue closed, try to complete bounty for {}",
            issue.url
        );

        let bounty = BountySdk::new()?.get_bounty(
            &self.domain.owner,
            &self.domain.sub_domain_name,
            &issue.id.0,
        )?;

        // get the top 150 comments on the issue
        let page_comments = self
            .gh
            .as_ref()
            .unwrap()
            .issues(&self.domain.owner, &self.domain.sub_domain_name)
            .list_comments(issue.number as u64)
            .per_page(150)
            .send()
            .await
            .map_err(|err| SBError::CommentsNotFound("closed issues".to_string(), err.to_string()))?
            .take_items();

        // try to get the comment body. If no closing comment -> return
        let comment_body = self.try_get_closing_comment(issue, &page_comments).await?;

        let solvers = get_solvers(
            &issue.user.id.to_string(),
            &comment_body,
            &issue.id,
            &bounty.mint,
        )
        .await?;

        let (bounty, sig) = BountySdk::new()?.complete_bounty(
            &self.domain.owner,
            &self.domain.sub_domain_name,
            &issue.id.0,
            &solvers,
            &bounty.mint,
        )?;

        // try post bounty statys
        let mut relayer_comments_iter = page_comments
            .iter()
            .filter(|comment| is_relayer_login(&comment.user.login).unwrap());
        let status = self.create_bounty_status_text(&bounty, Some(&sig))?;
        self.try_post_bounty_status(
            &(issue.number as u64),
            &status,
            &relayer_comments_iter.collect(),
        )
        .await?;
        Ok(())
    }

    pub async fn update_issue_status(&self, issue: &Issue) -> Result<(), SBError> {
        let bounty = BountySdk::new()?.get_bounty(
            &self.domain.owner,
            &self.domain.sub_domain_name,
            &issue.id.0,
        )?;

        let comments: Vec<Comment> = self
            .gh
            .as_ref()
            .unwrap()
            .issues(&self.domain.owner, &self.domain.sub_domain_name)
            .list_comments(issue.number as u64)
            .per_page(150)
            .send()
            .await
            .map_err(|err| SBError::CommentsNotFound("open issues".to_string(), err.to_string()))?
            .take_items();

        let relayer_comments_iter = comments
            .iter()
            .filter(|comment| is_relayer_login(&comment.user.login).unwrap());

        log::info!("[relayer] Try to post comments to issue {}", issue.url);

        Ok(())
    }

    pub async fn handle_issue(&self, issue: &Issue) -> Result<(), SBError> {
        // get Status of Issue
        // 1. Open - try create bounty
        // 2. Closed -
        //  - pay out bounty if mentioned users
        //  - close bounty if no one mentioned
        log::info!("[relayer] issue {}, issue state {}", issue.id, issue.state,);
        if issue.state.eq("open") {
            // -> If open -> try to complete bounty
            match self.handle_open_issue(&issue).await {
                Ok(res) => res,
                Err(err) => {
                    log::warn!(
                        "Could not handle open issue for {}. Cause {}",
                        issue.url,
                        err
                    );
                }
            };
        } else {
            // -> If closed -> try to complete bounty
            match self.handle_closed_issue(&issue).await {
                Ok(res) => res,
                Err(err) => {
                    log::warn!(
                        "Could not handle closed issue for {}. Cause {}",
                        issue.url,
                        err
                    );
                }
            }
        }

        Ok(())
    }

    /// issues
    ///
    /// Handles the github issues
    pub async fn issues(&self) -> Result<(), SBError> {
        log::info!(
            "[relayer] Index github issue for domain={}, repo={} ",
            self.domain.owner,
        );

        let issue_handler = self.gh.as_ref().unwrap().issues(&self.domain.owner, "*");
        let mut issues = match issue_handler
            .list()
            .state(params::State::All)
            .per_page(100)
            .send()
            .await
        {
            Ok(val) => val,
            Err(err) => return Err(SBError::FailedToGetIssue(err.to_string())),
        };
        log::info!(
            "issues: {:?}",
            issues
                .clone()
                .into_iter()
                .map(|iss| iss.id)
                .collect::<Vec<IssueId>>()
        );

        let shared_self = Arc::new(self);
        let self_copy = shared_self.clone();
        loop {
            for issue in &issues
                .take_items()
                .iter()
                .filter(|&issue| {
                    issue
                        .created_at
                        .timestamp()
                        .ge(&(get_unix_time(60 * 60 * 24 * 2) as i64))
                })
                .collect::<Vec<&Issue>>()
            {
                // get Status of Issue
                // 1. Open - try create bounty
                // 2. Closed -
                //  - pay out bounty if mentioned users
                //  - close bounty if no one mentioned
                let local_self = &self_copy;
                local_self.handle_issue(issue).await.unwrap();
            }

            // move to next issue
            issues = match self
                .gh
                .as_ref()
                .unwrap()
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
