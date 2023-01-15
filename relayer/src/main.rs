pub mod jobs;
use std::ops::Mul;

use anchor_client::anchor_lang::Result;
pub use jobs::verification;
use log;
use octocrab::{self, models, params};
use regex::Regex;
use tokio;

#[derive(Clone)]
pub struct Domain {
    name: String,
    owner: String,
    sub_domain_name: String,
    bounty_type: String,
    num_fails: u64, // number of time failed to index the domain
}

#[derive(PartialEq, Eq)]
pub enum DomainType {
    Issues,
    Unknown,
}

impl Domain {
    pub fn new(name: &str, owner: &str, sub_domain_name: &str, bounty_type: &str) -> Domain {
        return Domain {
            name: name.to_string(),
            owner: owner.to_string(),
            sub_domain_name: sub_domain_name.to_string(),
            bounty_type: bounty_type.to_string(),
            num_fails: 0,
        };
    }

    pub fn get_type(&self) -> DomainType {
        match self.bounty_type.as_str() {
            "issues" => DomainType::Issues,
            _ => DomainType::Unknown,
        }
    }
}

pub struct Bounty {
    amount: u64,
    token_name: String,
    creator: String,
}

impl Bounty {
    /// try_create_bounty will try to store the
    /// bounty in the program
    pub fn try_create_bounty(&self) -> Result<()> {
        Ok(())
    }
}

pub fn try_fetch_indexable_domains() -> Result<Vec<Domain>> {
    let test_domain = Domain {
        name: "github".to_string(),
        owner: "sandblizzard".to_string(),
        sub_domain_name: "rewards_v1".to_string(),
        bounty_type: "issue".to_string(),
        num_fails: 0,
    };
    let search_domains: Vec<Domain> = [test_domain].to_vec();
    Ok(search_domains)
}

/// get_bounty tries to extract the bounty from
/// the body of a potential bounty item
pub fn get_bounty(creator: &str, text: &str) -> Option<Bounty> {
    let re = Regex::new(r"\$(.+)\$/").unwrap();
    let captures = match re.captures(text) {
        Some(bounty) => bounty,
        None => return None,
    };

    let first_capture = match captures.get(1) {
        Some(capture) => capture,
        None => return None,
    };

    let inner_capture: String = first_capture.as_str().replace("$", "");
    let captures_items: Vec<&str> = inner_capture.split(":").collect();
    if inner_capture.len() != 2 {
        None
    } else {
        // convert amount in floating like 10.10 to 10.10x10^decimals u64
        let amount = captures_items[0].to_string().parse::<f64>().unwrap();
        // FIXME: fetch
        let decimals = 6;
        let amount_tokens = amount.mul(10_u32.pow(decimals) as f64).floor() as u64;

        // return Bounty
        Some(Bounty {
            amount: amount_tokens,
            token_name: captures_items[1].to_string(),
            creator: creator.to_string(),
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let gh = octocrab::instance();

    let search_domains = try_fetch_indexable_domains().unwrap();
    for domain in &search_domains {
        log::info!("[relayer] try index: {}", domain.name);
        if domain.get_type() == DomainType::Issues {
            let mut issues = gh
                .issues(&domain.name, &domain.sub_domain_name)
                .list()
                .state(params::State::Open)
                .send()
                .await
                .unwrap();
            loop {
                for issue in &issues {
                    log::info!("[relayer] found issue id={} ", issue.id);
                    // index the bounty information
                    let bounty =
                        get_bounty(&issue.user.id.to_string(), issue.body.as_ref().unwrap())
                            .unwrap();
                    // try to create the bounty
                    bounty.try_create_bounty().unwrap();
                }

                // move to next issue
                issues = match gh
                    .get_page::<models::issues::Issue>(&issues.next)
                    .await
                    .unwrap()
                {
                    Some(next_page) => next_page,
                    None => break,
                }
            }
        }
    }
    Ok(())
}
