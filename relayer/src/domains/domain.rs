use async_trait::async_trait;
use octocrab::models::Repository;
use std::result::Result;

use super::{github::Github, utils::SBError};

#[derive(Clone)]
pub struct Domain {
    pub name: String,
    pub owner: String,
    pub repos: Vec<Repository>,
    pub access_token_url: String,
    pub bounty_type: String,
    pub num_fails: u64, // number of time failed to index the domain
}

#[derive(PartialEq, Eq)]
pub enum DomainType {
    Issues,
    Unknown,
}

#[async_trait]
pub trait DomainHandler {
    fn name(&self) -> String;
    async fn handle(&self) -> Result<(), SBError>;
}

impl Domain {
    pub fn new(name: &str, owner: &str, _sub_domain_name: &str, bounty_type: &str) -> Domain {
        Domain {
            name: name.to_string(),
            owner: owner.to_string(),
            repos: Vec::new(),
            access_token_url: "".to_string(),
            bounty_type: bounty_type.to_string(),
            num_fails: 0,
        }
    }

    pub async fn get_type(&self) -> Result<Box<dyn DomainHandler>, SBError> {
        match self.bounty_type.as_str() {
            "issue" => {
                let github = Github::new(self).await?;
                Ok(Box::new(github))
            }
            "pull_request" => {
                let github = Github::new(self).await?;
                Ok(Box::new(github))
            }
            _ => Err(SBError::UndefinedBountyType(format!(
                "bounty_type = {} not defined",
                self.bounty_type
            ))),
        }
    }
}
