use core::fmt;
use regex::Regex;
use std::result::Result;
use std::{fmt::Debug, ops::Mul};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SBError {
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
}

pub struct Bounty {
    pub amount: u64,
    pub token_name: String,
    pub creator: String,
    pub id: u64,
}

impl Bounty {
    /// try_create_bounty will try to store the
    /// bounty in the program
    pub fn try_create_bounty(&self) -> Result<(), SBError> {
        log::info!("[relayer] Try create bounty with id={}", self.id);
        Ok(())
    }
}

/// get_bounty tries to extract the bounty from
/// the body of a potential bounty item
///
/// Assume bounty in the form $bonk:10.10$
pub fn get_bounty(creator: &str, text: &str, id: &u64) -> Result<Bounty, SBError> {
    let re = Regex::new(r"\$(.+)\$").unwrap();
    let captures = match re.captures(text) {
        Some(bounty) => bounty,
        None => {
            return Err(SBError::CouldNotGetBountyCapture(
                "could not get text capture".to_string(),
            ))
        }
    };

    let first_capture = match captures.get(1) {
        Some(capture) => capture,
        None => {
            return Err(SBError::CouldNotGetBountyCapture(
                "capture was not found".to_string(),
            ))
        }
    };

    let inner_capture: String = first_capture.as_str().replace("$", "");
    let captured_items: Vec<&str> = inner_capture.split(":").collect();
    if captured_items.len() != 2 {
        return Err(SBError::CouldNotFindInnerCapture(format!(
            "expected to capture 2 entitites. Instead captured {:?}",
            captured_items
        )));
    } else {
        // convert amount in floating like 10.10 to 10.10x10^decimals u64
        let amount = match captured_items[1].to_string().parse::<f64>() {
            Ok(amount) => amount,
            Err(err) => {
                return Err(SBError::FailedToConvertStringToNumber(format!(
                    "string={}. Cause: {}",
                    captured_items[0].to_string(),
                    err
                )))
            }
        };
        // FIXME: fetch
        let decimals = 6;
        let amount_tokens = amount.mul(10_u32.pow(decimals) as f64).floor() as u64;

        // return Bounty
        Ok(Bounty {
            amount: amount_tokens,
            token_name: captured_items[0].to_string(),
            creator: creator.to_string(),
            id: *id,
        })
    }
}

#[cfg(test)]
mod test {
    use super::get_bounty;

    #[test]
    pub fn test_get_bounty() {
        let text = "## About
        A bounty contract is needed to reward users for their bounty completion
        
        rewards
        $bonk:10$";

        let owner = "123";
        let id = 1;
        let bounty = get_bounty(owner, text, &id).unwrap();
        assert_eq!(bounty.amount, 10000000);
        assert_eq!(bounty.creator, owner);
    }
}
