use std::fmt::Debug;
use std::result::Result;
use std::{env, time};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum SBError {
    #[error("{0} Octocrab request failed. Cause {1}")]
    FailedOctocrabRequest(String, String),

    #[error("Github issue is not closed")]
    IssueNotClosed,

    #[error("{0}  Github comments not found. Reason {1}")]
    CommentsNotFound(String, String),

    #[error("{0}  Github comments not found. Reason {1}")]
    CommentNotFound(String, String),

    #[error("{0} Failed to comment. Reason {1}")]
    FailedToComment(String, String),

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

    #[error("bounty {0} was not found in the state. Cause {1}")]
    BountyDoesNotExistInState(String, String),

    #[error("Failed to load keypair from file. Cause {0}")]
    FailedToLoadKeypair(String),

    #[error("key {0} not found in environment")]
    KeyNotFound(String),

    #[error("could not get githib connection. Cause {0}")]
    FailedToGetGithubInstallations(String),

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

    #[error("{0} Could not convert {1} to {2}. Cause {3}")]
    CouldNotConvert(String, String, String, String),

    #[error("{0} Could not get env key {1}. Cause {2}")]
    CouldNotGetEnvKey(String, String, String),

    #[error("Failed to get {0}. Cause {1}")]
    FailedToGetAccount(String, String),

    #[error("{0} Need at least one solver ")]
    NeedAtLeastOneSolver(String),

    #[error("{0} Failed to parse token for issue {1} ")]
    FailedToParseBounty(String, String),

    #[error("{0} Failed fetch token information from {1} for {2}. Cause: {3} ")]
    FailedToFetchTokenInformation(String, String, String, String),
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
        Err(_err) => {
            log::debug!(
                "get_key_from_env: could not get key {}. Will try to get it from the environment",
                key
            );
        }
    }

    match env::var(key) {
        Ok(token) => return Ok(token.replace("\n", "")),
        Err(_err) => return Err(SBError::KeyNotFound(key.to_string())),
    }
}

pub fn get_unix_time(seconds_ago: u64) -> u64 {
    time::SystemTime::now()
        .checked_sub(time::Duration::new(seconds_ago, 0))
        .unwrap()
        .duration_since(time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
