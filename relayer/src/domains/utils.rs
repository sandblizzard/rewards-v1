use std::env;
use std::fmt::Debug;
use std::result::Result;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum SBError {
    #[error("Github issue is not closed")]
    IssueNotClosed,

    #[error("{0}  Github comments not found. Reason {1}")]
    CommentsNotFound(String, String),

    #[error("{0}  Github comments not found. Reason {1}")]
    CommentNotFound(String, String),

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

    #[error("{0} failed to get bounty from text. Reason {1}")]
    FailedToCompleteBounty(String, String),

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

    /// verification
    #[error("{0} failed to get verification file. Reason: {1}")]
    FailedToGetVerficationFile(String, String),

    #[error("{0} could not open local verification file. Reason: {1}")]
    FailedToOpenVerificationFile(String, String),

    #[error("{0} could not parse file. Reason: {1}")]
    FailedToParseFile(String, String),

    #[error("{0} Expected {1} found {2}")]
    UnexpectedNumberOfElements(String, u16, u16),

    #[error("{0} Username {1}, address {2}")]
    SandblizzardUserAlreadyExists(String, String, String),

    #[error("{0} Could not find user {1} among minted NFTs")]
    CouldNotFindUser(String, String),
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
