use octocrab::models::issues::Comment;

use crate::{
    bounty_proto::{get_solvers, BountyProto},
    bounty_sdk::BountySdk,
    domains::{
        github::utils::{comment_contains_signing_link, create_bounty_status_text},
        utils::{get_key_from_env, SBError},
    },
    external::{get_connection, is_relayer_login},
};

use super::utils::contains_bounty_status;

#[derive(Debug)]
pub struct SBIssue {
    pub id: u64,
    pub creator: String,
    pub access_token_url: String,
    pub owner: String,
    pub repo: String,
    pub number: i64,
    pub url: String,
    pub state: String,
    pub body: Option<String>,
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
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
        let bounty = match BountyProto::new_bounty_proto(&self.creator, issue_body, &self.id).await
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
            .any(|comment| contains_bounty_status(comment, status))
        {
            Ok(())
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
                    .any(|comment| comment_contains_signing_link(comment).unwrap());
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

    pub async fn handle(&self) -> Result<String, SBError> {
        if self.state.eq("open") {
            // -> If open -> try to complete bounty
            match self.open_issue().await {
                Ok(_res) => return Ok(self.url.clone()),
                Err(err) => {
                    log::warn!(
                        "Could not handle open issue for {}. Cause {}",
                        self.url,
                        err
                    );
                    return Err(err);
                }
            };
        } else {
            // -> If closed -> try to complete bounty
            match self.close_issue().await {
                Ok(_res) => return Ok(self.url.clone()),
                Err(err) => {
                    log::warn!(
                        "Could not handle closed issue for {}. Cause {}",
                        self.url,
                        err
                    );
                    return Err(err);
                }
            }
        }
    }
}
