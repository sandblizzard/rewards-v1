use anchor_lang::prelude::*;

#[account]
pub struct Relayer {
    pub bump: u8,
    pub bump_seed: [u8; 1],
    pub owner: Pubkey,
    pub active: bool,
}

impl Relayer {
    pub fn initialize(&mut self, owner: &Pubkey, bump: &u8) -> Result<()> {
        self.owner = *owner;
        self.bump_seed = [*bump; 1];
        self.bump = *bump;
        self.active = true;
        Ok(())
    }

    pub fn deactive(&mut self) -> Result<()> {
        self.active = false;
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
