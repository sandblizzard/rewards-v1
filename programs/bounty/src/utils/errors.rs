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

    #[msg("wrong mint for fee collector")]
    WrongFeeCollectorMint,

    #[msg("fee collector does not match protocol fee collector")]
    WrongProtocolFeeCollector,

    #[msg("invalid denomination fee collector")]
    WrongDenominationFeeCollector,

    #[msg("invalid denomination mint")]
    WrongDenominationMint,

    #[msg("Account is not signer")]
    AccountIsNotSigner,

    #[msg("Account is not active")]
    AccountNotActive,

    #[msg("Domain is not active")]
    DomainNotActive,
}
