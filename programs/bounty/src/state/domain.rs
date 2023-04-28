use anchor_lang::prelude::*;

/// Domain is the domain to be indexed
/// ex: if the domain is github/sandblizzard/rewards_v1 then
/// bounty_type = issues/pull_request
/// platform: github    
/// owner: <user
/// sub_domain: sandblizzard
/// repo: rewards_v1
#[account]
pub struct Domain {
    /// bump is used to sign transactions
    pub bump: u8,

    pub active: bool,

    // type of bounty such as issues, pull_request etc
    pub domain_type: String,

    /// platform is the domain such as GitHub
    pub platform: String,

    /// owner of the domain, could be an individual or dao
    /// it's the user who manage the domain
    pub owner: Pubkey,

    /// sub_domain is the identifier within the domain
    ///
    pub sub_domain: String,

    /// repo is the repo within the domain
    pub repo: String,
}

impl Domain {
    pub fn initialize(
        &mut self,
        domain_type: &str,
        sub_domain: &str,
        platform: &str,
        repo: &str,
        owner: &Pubkey,
    ) -> Result<()> {
        self.domain_type = domain_type.to_string();
        self.platform = platform.to_string();
        self.owner = *owner;
        self.active = true;
        self.sub_domain = sub_domain.to_string();
        self.repo = repo.to_string();
        Ok(())
    }

    pub fn deactivate(&mut self) -> Result<()> {
        self.active = false;
        Ok(())
    }
}
