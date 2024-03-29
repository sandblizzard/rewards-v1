use bounty_sdk::utils::SBError;
use coingecko;
use std::result::Result;

/// get_token_address returns the token address from coingecko
pub async fn get_token_address(
    token_name: &str,
    cluster: &str,
) -> Result<(String, String), SBError> {
    if cluster.eq("mainnet") {
        let api = coingecko::CoinGeckoClient::new("https://api.coingecko.com");
        let coin = match api
            .coin(token_name, false, false, false, false, false, false)
            .await
        {
            Ok(coin) => coin,
            Err(err) => {
                return Err(SBError::FailedToFetchTokenInformation(
                    "get_token_address".to_string(),
                    "coingecko".to_string(),
                    token_name.to_string(),
                    err.to_string(),
                ))
            }
        };
        let address = match coin.contract_address {
            Some(addr) => addr,
            None => {
                return Err(SBError::FailedToFetchTokenInformation(
                    "get_token_address".to_string(),
                    "coingecko".to_string(),
                    token_name.to_string(),
                    "Missing contract address".to_string(),
                ))
            }
        };

        Ok((address, coin.name))
    } else {
        Ok((
            "sandphoQsRiNd85VgRrdSXdhS56d58Xa9iDKwdnKfWR".to_string(),
            "sandBonk".to_string(),
        ))
    }
}
