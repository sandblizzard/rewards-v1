use std::{collections::HashMap, fmt::format};

use anchor_client::solana_sdk::pubkey::Pubkey;
use reqwest::header::{self, HeaderMap};

use crate::domains::utils::{get_key_from_env, SBError};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SandblizzardAttributes {
    overall: String,
    clout: String,
    credibility: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UnderdogCollection {
    name: String,
    description: String,
    image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<SandblizzardAttributes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mint_address: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UnderdogNFTs {
    results: Vec<UnderdogNFT>,
    page: u64,
    limit: u64,
    #[serde(rename = "totalPages")]
    total_pages: u64,
    #[serde(rename = "totalResults")]
    total_results: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct UnderdogNFT {
    name: String,
    description: String,
    image: String,
    attributes: SandblizzardAttributes,
    managed: bool,
    #[serde(rename = "ownerAddress")]
    owner_address: String,
    #[serde(rename = "collectionAddress")]
    collection_address: String,
    #[serde(rename = "mintAddress", skip_serializing_if = "Option::is_none")]
    mint_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateUnderdogObjectResponse {
    #[serde(rename = "mintAddress")]
    pub mint_address: String,
    #[serde(rename = "jobId")]
    pub job_id: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetUnderdogCollectionResponse {
    #[serde(rename = "mintAddress")]
    mint_address: String,
    name: String,
    description: String,
    image: String,
    status: String,
    #[serde(rename = "ownerAddress")]
    owner_address: String,
    nfts: UnderdogNFTs,
}

pub fn get_sandblizzard_collection() -> Result<Pubkey, SBError> {
    Ok(Pubkey::default())
}

///
pub fn get_bounty_user(username: &str) -> Result<(), SBError> {
    Ok(())
}

impl UnderdogCollection {
    pub fn new(name: String, description: String, image_url: String) -> UnderdogCollection {
        UnderdogCollection {
            name,
            description,
            image: image_url,
            attributes: None,
            mint_address: None,
        }
    }

    pub fn init_attributes(&self) -> Result<SandblizzardAttributes, SBError> {
        Ok(SandblizzardAttributes {
            overall: "0".to_string(),
            clout: "0".to_string(),
            credibility: "0".to_string(),
        })
    }

    /// get_collection gets the collection
    /// based on collection_address which needs to be
    /// stores somewhere
    pub async fn get_collection(&self) -> Result<GetUnderdogCollectionResponse, SBError> {
        // FIXME: call program to retrieve collection mint
        // The collection is created on `bounty` program initialization
        // thus the address should be found in the bounty config
        let collection_address = get_sandblizzard_collection()?;

        let client = reqwest::Client::new();

        let auth_token = get_key_from_env("UNDERDOG_KEY")?;

        let url = format!(
            "https://api.underdogprotocol.com/v1/collections/{}?limit={}",
            collection_address.to_string(),
            10,
        );
        let collection_response = client
            .get(url)
            .bearer_auth(&auth_token)
            .send()
            .await
            .map_err(|err| {
                SBError::FailedToRequestUnderdog("get_collection".to_string(), err.to_string())
            })?
            .json::<GetUnderdogCollectionResponse>()
            .await
            .map_err(|err| {
                SBError::FailedToDeserializeData("get_collection".to_string(), err.to_string())
            })?;

        Ok(collection_response)
    }

    /// create_collection creates a collection
    pub async fn create_collection(&self) -> Result<CreateUnderdogObjectResponse, SBError> {
        let client = reqwest::Client::new();

        let auth_token = get_key_from_env("UNDERDOG_KEY")?;

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        let collection_response = client
            .post("https://api.underdogprotocol.com/v1/collections")
            .headers(headers)
            .bearer_auth(&auth_token)
            .json(&self)
            .send()
            .await
            .map_err(|err| {
                SBError::FailedToRequestUnderdog("create_collection".to_string(), err.to_string())
            })?
            .json::<CreateUnderdogObjectResponse>()
            .await
            .map_err(|err| {
                SBError::FailedToDeserializeData("create_collection".to_string(), err.to_string())
            })?;

        Ok(collection_response)
    }

    /// mint_nft
    /// mints an underdog nft
    pub async fn mint_nft(
        &self,
        owner: Pubkey,
        username: &str,
    ) -> Result<CreateUnderdogObjectResponse, SBError> {
        let client = reqwest::Client::new();

        let auth_token = get_key_from_env("UNDERDOG_KEY")?;

        let underdog_nft = UnderdogNFT {
            name: username.to_string(),
            description: "Part of the Sandblizzard project".to_string(),
            image: "".to_string(),
            attributes: self.init_attributes()?,
            managed: false,
            owner_address: owner.to_string(),
            collection_address: self.mint_address.as_ref().unwrap().to_string(),
            mint_address: None,
            status: None,
        };

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let collection_response = client
            .post("https://api.underdogprotocol.com/v1/nfts")
            .headers(headers)
            .bearer_auth(&auth_token)
            .json(&underdog_nft)
            .send()
            .await
            .map_err(|err| {
                SBError::FailedToRequestUnderdog("mint_nft".to_string(), err.to_string())
            })?
            .json::<CreateUnderdogObjectResponse>()
            .await
            .map_err(|err| {
                SBError::FailedToDeserializeData("mint_nft".to_string(), err.to_string())
            })?;

        Ok(collection_response)
    }

    /// find_nft_from_username
    ///
    /// Takes nft name and searches through an entire collection.
    /// Returns the NFT is found, if not throws an error
    pub async fn find_nft_from_username(&self, name: &str) -> Result<UnderdogNFT, SBError> {
        let collection_address = get_sandblizzard_collection()?;
        let client = reqwest::Client::new();
        let auth_token = get_key_from_env("UNDERDOG_KEY")?;
        let mut params = HashMap::new();

        let mut cursor = 1;
        loop {
            log::info!("[find_nft_from_username] search with cursor {}", cursor);
            params.insert("page", cursor);
            let nfts_response = client
                .get(format!(
                    "https://api.underdogprotocol.com/v1/nfts?collectionAddress={}",
                    collection_address.to_string()
                ))
                .form(&params)
                .bearer_auth(&auth_token)
                .send()
                .await
                .map_err(|err| {
                    SBError::FailedToRequestUnderdog("nfts".to_string(), err.to_string())
                })?
                .json::<UnderdogNFTs>()
                .await
                .map_err(|err| {
                    SBError::FailedToDeserializeData("nfts".to_string(), err.to_string())
                })?;

            match nfts_response.results.iter().find(|nft| nft.name == name) {
                Some(nft) => return Ok(nft.clone()),
                None => (),
            }

            // move cursor
            cursor += 1;
        }
    }

    /// update_nft
    ///
    /// updates an nft if it is minted
    pub async fn update_nft(
        &self,
        name: &str,
        new_attr: SandblizzardAttributes,
    ) -> Result<(), SBError> {
        let mut underdog_nft = self.find_nft_from_username(name).await?;
        underdog_nft.attributes = new_attr;
        let nft_mint_address = match &underdog_nft.mint_address {
            Some(address) => address,
            None => return Err(SBError::UnderdogNFTNotMinted("update_nft".to_string())),
        };
        let client = reqwest::Client::new();
        let auth_token = get_key_from_env("UNDERDOG_KEY")?;

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        client
            .put(format!(
                "https://api.underdogprotocol.com/v1/nfts?{}",
                nft_mint_address
            ))
            .headers(headers)
            .bearer_auth(&auth_token)
            .json(&underdog_nft.clone())
            .send()
            .await
            .map_err(|err| {
                SBError::FailedToRequestUnderdog("update_nft".to_string(), err.to_string())
            })?
            .json::<CreateUnderdogObjectResponse>()
            .await
            .map_err(|err| {
                SBError::FailedToDeserializeData("update_nft".to_string(), err.to_string())
            })?;

        Ok(())
    }
}
