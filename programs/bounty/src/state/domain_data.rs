use anchor_lang::prelude::*;

/// DomainIdentifier
#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, PartialEq)]
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
}

impl DomainData {
    pub fn new(platform: &str, organization: &str, team: &str, domain_type: &str) -> Self {
        Self {
            platform: platform.to_string(),
            organization: organization.to_string(),
            team: team.to_string(),
            domain_type: domain_type.to_string(),
        }
    }

    pub fn generate_gh_url(&self) -> String {
        format!(
            "https://{}/{}/{}",
            self.platform, self.organization, self.team
        )
    }
}
