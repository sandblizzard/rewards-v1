use anchor_lang::prelude::*;

/// Domain is the domain to be indexed
#[account]
pub struct Domain {
    /// id is the identifier within the domain
    id: String,

    /// name is the domain such as GitHub
    name: String,

    /// owner of the domain, could be an individual or dao
    owner: Pubkey,

    /// repo is the repo within the domain
    repo: String,
}

impl Domain {
    pub fn initialize(&mut self, id: &str, name: &str, owner: &Pubkey) -> Result<()> {
        self.id = id.to_string();
        self.name = name.to_string();
        self.owner = *owner;
        Ok(())
    }
}
