use async_trait::async_trait;
use bounty::state::Domain;
use bounty_sdk::utils::SBError;
use std::result::Result;

use super::github::Github;

#[derive(Clone)]
pub struct RelayerDomain {
    pub domain_state: Domain,
    pub access_token_url: String,
}

impl RelayerDomain {
    pub fn new(domain_state: Domain, access_token_url: String) -> RelayerDomain {
        RelayerDomain {
            domain_state,
            access_token_url,
        }
    }

    pub async fn get_type(&self) -> Result<Box<dyn DomainHandler>, SBError> {
        match self.domain_state.domain.domain_type.as_str() {
            "issue" => {
                let github = Github::new(&self).await?;
                Ok(Box::new(github))
            }
            "pull_request" => {
                let github = Github::new(&self).await?;
                Ok(Box::new(github))
            }
            _ => Err(SBError::UndefinedBountyType(format!(
                "bounty_type = {} not defined",
                self.domain_state.domain.domain_type.as_str()
            ))),
        }
    }
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
