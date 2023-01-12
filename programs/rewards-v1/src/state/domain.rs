use anchor_lang::prelude::*;

/// Domain is the identifier
#[account]
pub struct Domain {
    /// id is the identifier within the domain
    id: String,

    /// name is the name of the domain
    name: String,

    /// owner of the domain, could be an individual or dao
    owner: Pubkey,
}

impl Domain {
    pub fn initialize(&mut self, id: &str, name: &str, owner: &Pubkey) -> Result<()> {
        self.id = id.to_string();
        self.name = name.to_string();
        self.owner = *owner;
        Ok(())
    }
}
