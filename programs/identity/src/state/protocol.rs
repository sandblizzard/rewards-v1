use anchor_lang::prelude::*;

#[account]
pub struct Protocol {
    owner: Pubkey,
}

impl Protocol {
    pub fn initialize(&mut self, owner: &Pubkey) -> Result<()> {
        self.owner = *owner;
        Ok(())
    }
}
