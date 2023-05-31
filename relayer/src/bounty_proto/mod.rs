use crate::external::{tokens, UnderdogCollection};
use anchor_client::solana_sdk::pubkey::*;

use bounty_sdk::utils::{get_key_from_env, SBError};
use regex::Regex;

/// Bounty is the SDK for the bounty program
use std::{result::Result, str::FromStr};
pub struct BountyProto {
    pub amount: Option<f64>,
    pub token_name: Option<String>,
    pub token_mint: Option<String>,
    pub creator: String,
    pub id: u64,
    pub solvers: Option<Vec<Pubkey>>,
    pub state: String,
}

impl BountyProto {
    /// get_bounty tries to extract the bounty from
    /// the body of a potential bounty item
    ///
    /// Assume bounty in the form $bonk:10.10$
    pub async fn new_bounty_proto(
        creator: &str,
        text: &str,
        id: &u64,
    ) -> Result<BountyProto, SBError> {
        let re = Regex::new(r"\$(.+)\$").unwrap();
        let captures = match re.captures(text) {
            Some(bounty) => bounty,
            None => {
                return Err(SBError::CouldNotGetBountyCapture(
                    text.to_string(),
                    "could not get text capture".to_string(),
                ))
            }
        };

        let first_capture = match captures.get(1) {
            Some(capture) => capture,
            None => {
                return Err(SBError::CouldNotGetBountyCapture(
                    text.to_string(),
                    "capture was not found".to_string(),
                ))
            }
        };

        let inner_capture: String = first_capture.as_str().replace('$', "");
        let captured_items: Vec<&str> = inner_capture.split(':').collect();
        if captured_items.len() != 2 {
            return Err(SBError::CouldNotFindInnerCapture(format!(
                "expected to capture 2 entitites. Instead captured {:?}",
                captured_items
            )));
        } else {
            let token = match captured_items[0].to_string().parse::<String>() {
                Ok(token) => token,
                Err(_err) => {
                    return Err(SBError::FailedToParseBounty(
                        "new_bounty_proto".to_string(),
                        id.to_string(),
                    ))
                }
            };

            let cluster = get_key_from_env("CLUSTER")?;
            let (token_mint, token_name) = tokens::get_token_address(&token, &cluster).await?;
            // convert amount in floating like 10.10 to 10.10x10^decimals u64
            let amount = match captured_items[1].to_string().trim().parse::<f64>() {
                Ok(amount) => amount,
                Err(err) => {
                    return Err(SBError::FailedToConvertStringToNumber(format!(
                        "string={}. Cause: {}",
                        captured_items[1].to_string(),
                        err
                    )))
                }
            };
            // return Bounty
            Ok(BountyProto {
                amount: Some(amount),
                token_name: Some(token_name),
                token_mint: Some(token_mint),
                creator: creator.to_string(),
                id: *id,
                solvers: None,
                state: "not_started".to_string(),
            })
        }
    }
}

/// get_solvers takes the issue close text and finds the mentioned users
pub async fn get_solvers(text: &str) -> Result<Vec<Pubkey>, SBError> {
    // find user names
    let re = Regex::new(r"@[.^\S]+").unwrap();
    let captures = match re.captures(text) {
        Some(bounty) => bounty,
        None => {
            return Err(SBError::CouldNotGetBountyCapture(
                text.to_string(),
                "could not get text capture".to_string(),
            ))
        }
    };

    let usernames: Vec<String> = captures
        .iter()
        .filter(|x| x.is_some())
        .map(|username| username.unwrap().as_str().replace('@', ""))
        .collect();

    let mut solvers = Vec::new();
    let collection_mint = get_key_from_env("SANDBLIZZARD_COLLECTION_ADDRESS")?;
    let underdog_api = UnderdogCollection::new(&collection_mint);
    for solver in usernames {
        let nft = underdog_api.find_nft_from_name(&solver).await.unwrap();
        let solver = Pubkey::from_str(&nft.owner_address).unwrap();

        // associated token address
        solvers.push(solver);
    }
    Ok(solvers)
}

#[cfg(test)]
mod test {

    // #[test]
    // pub fn test_get_bounty() {
    //     let text = "## About
    //     A bounty contract is needed to reward users for their bounty completion

    //     rewards
    //     $bonk:10$";

    //     let owner = "123";
    //     let id = 1;
    //     let bounty = get_bounty_proto(owner, text, &id).unwrap();
    //     assert_eq!(bounty.amount.unwrap(), 10.);
    //     assert_eq!(bounty.creator, owner);
    // }
}
