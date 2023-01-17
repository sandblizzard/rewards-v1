use std::ops::Mul;

use regex::Regex;

use super::utils::SBError;

pub struct Bounty {
    pub amount: Option<u64>,
    pub token_name: Option<String>,
    pub creator: String,
    pub id: u64,
    pub solvers: Option<Vec<String>>,
}

impl Bounty {
    /// try_create_bounty will try to store the
    /// bounty in the program
    pub fn try_create_bounty(&self) -> Result<(), SBError> {
        log::info!("[bounty] Try create bounty with id={}", self.id);

        // if bounty is new then
        Ok(())
    }

    /// try_complete_bounty will complete a bounty if solvers
    /// were specified, if not it is deemed cancelled
    pub fn try_complete_bounty(&self) -> Result<(), SBError> {
        log::info!(
            "[bounty] Try to complete bounty with id={}, for solvers: {:?}",
            self.id,
            self.solvers.as_ref().unwrap()
        );
        Ok(())
    }
}

/// get_solvers takes the issue close text and finds the mentioned users
pub fn get_solvers(creator: &str, text: &str, id: &u64) -> Result<Bounty, SBError> {
    // find user names
    let re = Regex::new(r"@[.^\S]+").unwrap();
    let captures = match re.captures(text) {
        Some(bounty) => bounty,
        None => {
            return Err(SBError::CouldNotGetBountyCapture(
                "could not get text capture".to_string(),
            ))
        }
    };

    let usernames: Vec<String> = captures
        .iter()
        .filter(|x| x.is_some())
        .map(|username| username.unwrap().as_str().replace("@", ""))
        .collect();
    Ok(Bounty {
        amount: None,
        token_name: None,
        creator: creator.to_string(),
        id: *id,
        solvers: Some(usernames),
    })
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
            amount: Some(amount_tokens),
            token_name: Some(captured_items[0].to_string()),
            creator: creator.to_string(),
            id: *id,
            solvers: None,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::domains::bounty::get_solvers;

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
        assert_eq!(bounty.amount.unwrap(), 10000000);
        assert_eq!(bounty.creator, owner);
    }

    #[test]
    pub fn test_get_solvers() {
        let text = "## About
        A bounty contract is needed to reward users for their bounty completion
        
        @0xksure";

        let owner = "123";
        let id = 1;
        let bounty = get_solvers(owner, text, &id).unwrap();
        assert_eq!(bounty.amount.is_none(), true);
        assert_eq!(bounty.solvers.unwrap().len(), 1);
    }
}
