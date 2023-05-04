use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

/// DomainIdentifier
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct DomainIdentifier {
    /// platform is the domain such as GitHub
    pub platform: String,

    /// domain is the identifier within the domain
    /// like sandblizzard
    /// FIXME: rename
    pub domain: String,

    /// sub_domain is the identifier within the domain
    /// like rewards_v1. This corresponds to the Bounty
    /// sub_domain
    pub sub_domain: String,

    // type of bounty such as issues, pull_request etc
    pub domain_type: String,

    /// Effective url path of platform URL
    pub url: Option<String>,
}

impl DomainIdentifier {
    pub fn new(
        platform: &str,
        domain: &str,
        sub_domain: &str,
        domain_type: &str,
        repo: Option<String>,
    ) -> Self {
        Self {
            platform: platform.to_string(),
            domain: domain.to_string(),
            sub_domain: sub_domain.to_string(),
            domain_type: domain_type.to_string(),
            url: repo,
        }
    }
}
