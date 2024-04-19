

use anchor_lang::prelude::*;


use crate::utils::BOUNTY_SEED;

#[account]
pub struct Protocol {
    pub bump: u8,

    pub bump_seed: [u8; 1],

    // owner of the protocol
    // it has the ability to update it
    pub owner: Pubkey,

    pub sand_mint: Pubkey,

    pub emission: u64,
}

impl Protocol {
    pub fn seeds(&self) -> [&[u8]; 2] {
        [BOUNTY_SEED, &self.bump_seed]
    }

    pub fn account_seed(&self) -> &[u8] {
        BOUNTY_SEED
    }

    pub fn initialize(&mut self, bump: &u8, owner: &Pubkey, mint: &Pubkey) -> Result<()> {
        self.owner = *owner;
        self.bump = *bump;
        self.bump_seed = [*bump; 1];
        self.sand_mint = *mint;
        self.emission = 1;
        Ok(())
    }

    pub fn update_emission(&mut self, emission: u64) -> Result<()> {
        self.emission = emission;
        Ok(())
    }

    pub fn calculate_mining_reward(&self, num_solvers: usize, decimals: u8) -> u64 {
        let token_decimals = 10u64.pow(decimals as u32);
        (self.emission * token_decimals)
            .checked_div(num_solvers as u64)
            .unwrap()
    }
}

//test
#[cfg(test)]
mod tests {
    use super::*;

    use anchor_lang::solana_program::pubkey::Pubkey;

    // test calculate mining reward
    #[test]
    fn test_calculate_mining_reward() {
        let protocol = Protocol {
            bump: 0,
            bump_seed: [0],
            owner: Pubkey::new_unique(),
            sand_mint: Pubkey::new_unique(),
            emission: 100,
        };

        let reward = protocol.calculate_mining_reward(10, 2);
        assert_eq!(reward, 1000);
    }

    // test initialize
    #[test]
    fn test_initialize() {
        let mut protocol = Protocol {
            bump: 0,
            bump_seed: [0],
            owner: Pubkey::new_unique(),
            sand_mint: Pubkey::new_unique(),
            emission: 100,
        };

        let owner = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let bump = 1;

        protocol.initialize(&bump, &owner, &mint).unwrap();

        assert_eq!(protocol.owner, owner);
        assert_eq!(protocol.sand_mint, mint);
        assert_eq!(protocol.emission, 1);
    }

    // test update emission
    #[test]
    fn test_update_emission() {
        let mut protocol = Protocol {
            bump: 0,
            bump_seed: [0],
            owner: Pubkey::new_unique(),
            sand_mint: Pubkey::new_unique(),
            emission: 100,
        };

        let emission = 200;

        protocol.update_emission(emission).unwrap();

        assert_eq!(protocol.emission, emission);
    }

    // test seeds
    #[test]
    fn test_seeds() {
        let protocol = Protocol {
            bump: 0,
            bump_seed: [0],
            owner: Pubkey::new_unique(),
            sand_mint: Pubkey::new_unique(),
            emission: 100,
        };

        let seeds = protocol.seeds();

        assert_eq!(seeds[0], BOUNTY_SEED);
        assert_eq!(seeds[1], &[0]);
    }

    // test account seed
    #[test]
    fn test_account_seed() {
        let protocol = Protocol {
            bump: 0,
            bump_seed: [0],
            owner: Pubkey::new_unique(),
            sand_mint: Pubkey::new_unique(),
            emission: 100,
        };

        let seed = protocol.account_seed();

        assert_eq!(seed, BOUNTY_SEED);
    }
}
