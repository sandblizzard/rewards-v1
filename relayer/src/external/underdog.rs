use std::collections::HashMap;

use anchor_client::solana_sdk::{pubkey::Pubkey, signature};
use reqwest::header::HeaderMap;

use crate::domains::utils::{get_key_from_env, SBError};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SandblizzardAttributes {
    overall: String,
    clout: String,
    credibility: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UnderdogCollection {
    mint_address: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateUnderdogCollection {
    name: String,
    description: String,
    image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<SandblizzardAttributes>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UnderdogNFTs {
    pub results: Vec<UnderdogNFT>,
    pub page: u64,
    pub limit: u64,
    #[serde(rename = "totalPages")]
    pub total_pages: u64,
    #[serde(rename = "totalResults")]
    pub total_results: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct UnderdogNFT {
    pub name: String,
    pub description: String,
    pub image: String,
    pub attributes: SandblizzardAttributes,
    #[serde(rename = "managed", default = "default_bool")]
    pub managed: bool,
    #[serde(rename = "ownerAddress")]
    pub owner_address: String,
    #[serde(rename = "collectionAddress")]
    pub collection_address: String,
    #[serde(rename = "mintAddress", skip_serializing_if = "Option::is_none")]
    pub mint_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateUnderdogObjectResponse {
    #[serde(rename = "mintAddress")]
    pub mint_address: String,
    #[serde(rename = "jobId", default = "default_string")]
    pub job_id: String,
}

fn default_string() -> String {
    "".to_string()
}
fn default_bool() -> bool {
    false
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

pub fn get_sandblizzard_collection() -> Result<String, SBError> {
    get_key_from_env("SANDBLIZZARD_COLLECTION_ADDRESS")
}

///
pub fn get_bounty_user(_username: &str) -> Result<(), SBError> {
    Ok(())
}

impl UnderdogCollection {
    pub fn new(mint_address: &str) -> UnderdogCollection {
        UnderdogCollection {
            mint_address: mint_address.to_string(),
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
    pub async fn create_collection(
        &self,
        name: &str,
        description: &str,
        image_url: &str,
        attributes: Option<SandblizzardAttributes>,
    ) -> Result<CreateUnderdogObjectResponse, SBError> {
        let client = reqwest::Client::new();

        let auth_token = get_key_from_env("UNDERDOG_KEY")?;

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let collection = CreateUnderdogCollection {
            name: name.to_string(),
            description: description.to_string(),
            image: image_url.to_string(),
            attributes: attributes,
        };
        let collection_response = client
            .post("https://api.underdogprotocol.com/v1/collections")
            .headers(headers)
            .bearer_auth(&auth_token)
            .json(&collection)
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
        owner: &str,
        username: &str,
    ) -> Result<CreateUnderdogObjectResponse, SBError> {
        // don't mint if NFT already exists
        match self.find_nft_from_name(username).await {
            Ok(res) => {
                return Err(SBError::SandblizzardUserAlreadyExists(
                    "mint_nft. Will not mint a new NFT".to_string(),
                    res.name.to_string(),
                    res.owner_address.to_string(),
                ))
            }
            Err(err) => {
                log::info!(
                    "[mint_nft] error from find_nft_from_name {}",
                    err.to_string()
                );
                if err.ne(&SBError::CouldNotFindUser("".to_string(), "".to_string())) {
                } else {
                    return Err(err);
                }
            }
        }
        log::info!("mint_nft: Mint nft for user {}", username.to_string());
        let client = reqwest::Client::new();

        let auth_token = get_key_from_env("UNDERDOG_KEY")?;

        let underdog_nft = UnderdogNFT {
            name: username.to_string(),
            description: "Part of the Sandblizzard project".to_string(),
            image: "https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/0bb24e44-ed4f-4c54-86f7-a5f53487720b/dbbp2hk-9d649885-2f66-478c-9d24-dea19bfe530e.png/v1/fill/w_1600,h_1547,q_75,strp/alolan_sandslash_by_pokemon_vector_art-dbbp2hk.png?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJ1cm46YXBwOjdlMGQxODg5ODIyNjQzNzNhNWYwZDQxNWVhMGQyNmUwIiwic3ViIjoidXJuOmFwcDo3ZTBkMTg4OTgyMjY0MzczYTVmMGQ0MTVlYTBkMjZlMCIsImF1ZCI6WyJ1cm46c2VydmljZTppbWFnZS5vcGVyYXRpb25zIl0sIm9iaiI6W1t7InBhdGgiOiIvZi8wYmIyNGU0NC1lZDRmLTRjNTQtODZmNy1hNWY1MzQ4NzcyMGIvZGJicDJoay05ZDY0OTg4NS0yZjY2LTQ3OGMtOWQyNC1kZWExOWJmZTUzMGUucG5nIiwid2lkdGgiOiI8PTE2MDAiLCJoZWlnaHQiOiI8PTE1NDcifV1dfQ.kNH2RFY20v4K010p3h6Nw6hx1L0br9jSLJbJi6LChvk".to_string(),
            attributes: self.init_attributes()?,
            managed: false,
            owner_address: owner.to_string(),
            collection_address: self.mint_address.clone(),
            mint_address: None,
            status: None,
        };

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let mint_nft_raw_response = client
            .post("https://api.underdogprotocol.com/v1/nfts")
            .headers(headers)
            .bearer_auth(&auth_token)
            .json(&underdog_nft)
            .send()
            .await
            .map_err(|err| {
                SBError::FailedToRequestUnderdog("mint_nft".to_string(), err.to_string())
            })?;

        log::debug!(
            "[relayer] mint_nft raw response: {:?} ",
            mint_nft_raw_response
        );

        let mint_nft_reponse = mint_nft_raw_response
            .json::<CreateUnderdogObjectResponse>()
            .await
            .map_err(|err| {
                SBError::FailedToDeserializeData("mint_nft".to_string(), err.to_string())
            })?;

        Ok(mint_nft_reponse)
    }

    /// find_nft_from_username
    ///
    /// Takes nft name and searches through an entire collection.
    /// Returns the NFT is found, if not throws an error
    pub async fn find_nft_from_name(&self, name: &str) -> Result<UnderdogNFT, SBError> {
        log::info!("[find_nft_from_name]: for name {}", name.to_string());
        let collection_address = get_sandblizzard_collection()?;
        let client = reqwest::Client::new();
        let auth_token = get_key_from_env("UNDERDOG_KEY")?;
        let mut params = HashMap::new();

        let mut cursor = 1;
        loop {
            log::info!("[name] search with cursor {}", cursor);
            params.insert("page", cursor);
            let nfts_response_raw = client
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
                })?;

            let nfts_response = nfts_response_raw
                .json::<UnderdogNFTs>()
                .await
                .map_err(|err| {
                    SBError::FailedToDeserializeData("nfts".to_string(), err.to_string())
                })?;

            match nfts_response.results.iter().find(|nft| nft.name == name) {
                Some(nft) => {
                    log::info!(
                        "[find_nft_from_username] found user {}",
                        nft.name.to_string()
                    );
                    return Ok(nft.clone());
                }
                None => (),
            }

            // move cursor
            cursor += 1;
            if cursor == nfts_response.total_pages {
                log::info!(
                    "[find_nft_from_username] cursor reached total pages {}",
                    nfts_response.total_pages
                );
                return Err(SBError::CouldNotFindUser(
                    "find_nft_from_name".to_string(),
                    name.to_string(),
                ));
            }
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
        let mut underdog_nft = self.find_nft_from_name(name).await?;
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
