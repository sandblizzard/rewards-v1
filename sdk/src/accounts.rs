use spl_associated_token_account::solana_program::pubkey::Pubkey;

/// Contains methods useful to derive PDAs
///

/// get_bounty_pda
pub fn get_bounty_pda(issue_id: &u64) -> (Pubkey, u8) {
    let bounty_pda = Pubkey::find_program_address(
        &[
            bounty::utils::BOUNTY_SEED.as_bytes(),
            issue_id.to_string().as_bytes(),
        ],
        &bounty::ID,
    );
    bounty_pda
}

pub fn get_protocol_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[bounty::utils::BOUNTY_SEED.as_bytes()], &bounty::id())
}

pub fn get_relayer_pda(relayer_address: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            bounty::utils::BOUNTY_SEED.as_bytes(),
            relayer_address.to_bytes().as_ref(),
        ],
        &bounty::id(),
    )
}

pub fn get_escrow_pda(bounty_pk: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[bounty_pk.to_bytes().as_ref()], &bounty::id())
}

/// domain PDA
pub fn get_domain_pda(
    platform: &str,
    organization: &str,
    team: &str,
    domain_type: &str,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            bounty::utils::BOUNTY_SEED.as_bytes(),
            platform.as_bytes(),
            organization.as_bytes(),
            team.as_bytes(),
            domain_type.as_bytes(),
        ],
        &bounty::id(),
    )
}

pub fn get_bounty_denomination_pda(mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            bounty::utils::BOUNTY_SEED.as_bytes(),
            bounty::utils::DENOMINATION_SEED.as_bytes(),
            mint.to_bytes().as_ref(),
        ],
        &bounty::id(),
    )
}

pub fn get_fee_collector_pda(mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            bounty::utils::BOUNTY_SEED.as_bytes(),
            bounty::utils::FEE_COLLECTOR_SEED.as_bytes(),
            mint.to_bytes().as_ref(),
        ],
        &bounty::id(),
    )
}

pub fn get_sand_token_pda(sand_mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            bounty::utils::BOUNTY_SEED.as_bytes(),
            sand_mint.to_bytes().as_ref(),
        ],
        &bounty::id(),
    )
}
