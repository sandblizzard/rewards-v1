use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

/// DomainIdentifier
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct DomainData {
    /// platform is the domain such as GitHub
    pub platform: String,

    /// organization is the identifier within the domain
    /// like sandblizzard
    /// FIXME: rename
    pub organization: String,

    /// team is the identifier within the domain
    /// like rewards_v1. This corresponds to the Bounty
    pub team: String,

    // type of bounty such as issues, pull_request etc
    pub domain_type: String,

    /// Effective url path of platform URL
    pub url: Option<String>,
}

impl DomainData {
    pub fn new(
        platform: &str,
        organization: &str,
        team: &str,
        domain_type: &str,
        repo: Option<String>,
    ) -> Self {
        Self {
            platform: platform.to_string(),
            organization: organization.to_string(),
            team: team.to_string(),
            domain_type: domain_type.to_string(),
            url: repo,
        }
    }
}
