use core::fmt;
use regex::Regex;
use std::result::Result;
use std::{fmt::Debug, ops::Mul};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum SBError {
    #[error("could not create signing link. Reson:{0}")]
    CouldNotGenerateSigningLink(String),

    #[error("could not get path. Reason {0}")]
    CouldNotGetPath(String),

    #[error("github token not set")]
    GithubTokenNotSet,

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

    /// PULLS
    #[error("can't get pull requests. Reason {0}")]
    FailedToFetchPulls(String),
}
