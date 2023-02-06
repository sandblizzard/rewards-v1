use anchor_client::{Client, Program};
use async_trait::async_trait;
use octocrab::models::InstallationId;
use std::{rc::Rc, result::Result, sync::Arc};
use tokio::sync::Mutex;

use crate::bounty_sdk::BountySdk;

use super::{github::Github, utils::SBError};

#[derive(Clone)]
pub struct Domain {
    pub name: String,
    pub owner: String,
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
    pub fn new(name: &str, owner: &str, sub_domain_name: &str, bounty_type: &str) -> Domain {
        return Domain {
            name: name.to_string(),
            owner: owner.to_string(),
            access_token_url: "".to_string(),
            bounty_type: bounty_type.to_string(),
            num_fails: 0,
        };
    }

    pub async fn get_type(&self) -> Result<Box<dyn DomainHandler>, SBError> {
        match self.bounty_type.as_str() {
            "issue" => {
                let github = Github::new(&self).await?;
                Ok(Box::new(github))
            }
            _ => Err(SBError::UndefinedBountyType(format!(
                "bounty_type = {} not defined",
                self.bounty_type
            ))),
        }
    }
}
