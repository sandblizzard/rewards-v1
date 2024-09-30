use anchor_lang::prelude::*;

use super::domain_data::DomainData;

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

    /// owner of the domain, could be an individual or dao
    /// it's the user who manage the domain
    pub owner: Pubkey,

    pub installation_id: u32,

    /// FIXME: Rename
    pub data: DomainData,
}

impl Domain {
    pub fn initialize(
        &mut self,
        domain_type: &str,
        organization: &str,
        team: &str,
        platform: &str,
        owner: &Pubkey,
        bump: &u8,
        installation_id: u32,
    ) -> Result<()> {
        self.data.domain_type = domain_type.to_string();
        self.data.organization = organization.to_string();
        self.data.platform = platform.to_string();

        self.owner = *owner;
        self.active = true;
        self.data.team = team.to_string();
        self.bump = *bump;
        self.installation_id = installation_id;
        Ok(())
    }

    pub fn deactivate(&mut self) -> Result<()> {
        self.active = false;
        Ok(())
    }

    pub fn get_type(&self) -> String {
        self.data.domain_type.clone()
    }
}
