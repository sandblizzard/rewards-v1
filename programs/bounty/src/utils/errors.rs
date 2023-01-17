use anchor_lang::prelude::*;
use std::num::TryFromIntError;

#[error_code]
#[derive(PartialEq)]
pub enum BlizzardError {
    #[msg("bounty can not be reinitialized")]
    CanNotReinitBounty,

    #[msg("signer missing auth to complete bounty")]
    NotAuthToCompleteBounty,
}
