use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct BountySolution {
    pub id: u64,
    pub bump_array: [u8; 1],
    pub bounty: Pubkey,
    pub solver: Pubkey,
    pub solution: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl BountySolution {
    pub fn initialize(
        &mut self,
        bump: &u8,
        id: u64,
        bounty: &Pubkey,
        solver: &Pubkey,
        solution: &str,
    ) -> Result<()> {
        self.id = id;
        self.bump_array = [*bump; 1];
        self.bounty = *bounty;
        self.solver = *solver;
        self.solution = solution.to_string();
        self.created_at = Clock::get()?.unix_timestamp;
        self.updated_at = Clock::get()?.unix_timestamp;

        Ok(())
    }
}
