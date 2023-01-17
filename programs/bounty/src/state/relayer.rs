use anchor_lang::prelude::*;

#[account]
pub struct Relayer {
    pub bump: u8,
    pub owner: Pubkey,
}

impl Relayer {
    pub fn initialize(&mut self, owner: &Pubkey, bump: u8) -> Result<()> {
        self.owner = *owner;
        self.bump = bump;
        Ok(())
    }

    pub fn is_owner(&self, user: &Pubkey) -> bool {
        self.owner.eq(user)
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
