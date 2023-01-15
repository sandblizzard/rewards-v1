use anchor_lang::prelude::*;

#[account]
pub struct Relayer {
    owner: Pubkey,
}

impl Relayer {
    pub fn initialize(&mut self, owner: &Pubkey) -> Result<()> {
        self.owner = *owner;
        Ok(())
    }
}

#[account]
pub struct Relayers {
    relayers: Vec<Pubkey>,
}

impl Relayers {
    pub fn initialize(&mut self) -> Result<()> {
        self.relayers = Vec::new();
        Ok(())
    }
}
