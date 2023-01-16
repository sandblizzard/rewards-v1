use async_trait::async_trait;
use std::{
    future::{self, Future},
    io,
    result::Result,
};

use super::{github::Github, utils::SBError};

#[derive(Clone)]
pub struct Domain {
    pub name: String,
    pub owner: String,
    pub sub_domain_name: String,
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
    async fn handle(&self) -> Result<(), SBError>;
    fn name(&self) -> String;
}

impl Domain {
    pub fn new(name: &str, owner: &str, sub_domain_name: &str, bounty_type: &str) -> Domain {
        return Domain {
            name: name.to_string(),
            owner: owner.to_string(),
            sub_domain_name: sub_domain_name.to_string(),
            bounty_type: bounty_type.to_string(),
            num_fails: 0,
        };
    }

    pub fn get_type(&self) -> Result<Box<dyn DomainHandler>, SBError> {
        match self.bounty_type.as_str() {
            "issue" => Ok(Box::new(Github {
                domain: self.clone(),
            })),
            _ => Err(SBError::UndefinedBountyType(format!(
                "bounty_type = {} not defined",
                self.bounty_type
            ))),
        }
    }
}
