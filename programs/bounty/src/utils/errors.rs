use anchor_lang::prelude::*;

#[error_code]
#[derive(PartialEq)]
pub enum BlizzardError {
    #[msg("bounty can not be reinitialized")]
    CanNotReinitBounty,

    #[msg("signer missing auth to complete bounty")]
    NotAuthToCompleteBounty,

    #[msg("signer missing auth to release escrow")]
    NotAuthToReleaseEscrow,

    #[msg("at least one receiver needs to be specified")]
    MissingReceiverTokenAccounts,
}
