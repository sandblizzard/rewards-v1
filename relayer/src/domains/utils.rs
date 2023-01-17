use std::env;
use std::result::Result;
use std::{fmt::Debug};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum SBError {
    #[error("could not create signing link. Reson:{0}")]
    CouldNotGenerateSigningLink(String),

    #[error("could not get path. Reason {0}")]
    CouldNotGetPath(String),

    #[error("undefined bounty type. Reason {0}")]
    UndefinedBountyType(String),

    #[error("failed to get issue. Reason {0}")]
    FailedToGetIssue(String),

    #[error("failed to get bounty from text. Reason {0}")]
    FailedToFindBounty(String),

    #[error("could not get bounty capture. Reason {0}")]
    CouldNotGetBountyCapture(String),

    #[error("inner capture not found. Reason {0}")]
    CouldNotFindInnerCapture(String),

    #[error("could not convert string to number. Reason {0}")]
    FailedToConvertStringToNumber(String),

    #[error("empty option. Reason {0}")]
    EmptyOption(String),

    #[error("bounty allready exists")]
    BountyExists,

    #[error("key {0} not found in environment")]
    KeyNotFound(String),

    /// PULLS
    #[error("can't get pull requests. Reason {0}")]
    FailedToFetchPulls(String),

    /// underdog
    #[error("failed to get {0}. Reason {1}")]
    FailedToRequestUnderdog(String, String),

    #[error("failed to deserialize data {0}. Reason {1}")]
    FailedToDeserializeData(String, String),

    #[error("underdog nft not minted. Source {0}")]
    UnderdogNFTNotMinted(String),
}

/// get_key_from_env
///
/// tries to find `key` in the local .env file and returns it
pub fn get_key_from_env(key: &str) -> Result<String, SBError> {
    // assumes .env
    let path = match env::current_dir().and_then(|a| Ok(a.as_path().join(".env"))) {
        Ok(res) => res,
        Err(err) => return Err(SBError::CouldNotGetPath(err.to_string())),
    };
    match dotenv::from_path(&path) {
        Ok(_) => (),
        Err(err) => {
            return Err(SBError::CouldNotGetPath(format!(
                "path={:?}, cause: {}",
                &path,
                err.to_string()
            )))
        }
    }

    match env::var(key) {
        Ok(token) => return Ok(token.replace("\n", "")),
        Err(_err) => return Err(SBError::KeyNotFound(key.to_string())),
    }
}
