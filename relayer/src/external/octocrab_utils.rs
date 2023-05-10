use bounty_sdk::utils::{get_key_from_env, SBError};
use octocrab::{models, params::apps::CreateInstallationAccessToken, Octocrab};

pub fn is_relayer_login(login: &str) -> Result<bool, SBError> {
    let app_login = get_key_from_env("GITHUB_APP_LOGIN")?;
    Ok(login.eq(&app_login))
}

pub fn get_octocrab_instance() -> Result<Octocrab, SBError> {
    let github_key = get_key_from_env("GITHUB_KEY")?;
    let github_id = get_key_from_env("GITHUB_ID")?;
    let app_id = github_id.parse::<u64>().unwrap().into();
    let key = jsonwebtoken::EncodingKey::from_rsa_pem(github_key.as_bytes()).unwrap();
    match Octocrab::builder().app(app_id, key).build() {
        Ok(gh) => Ok(gh),
        Err(err) => Err(SBError::FailedOctocrabRequest(
            "get_octocrab_instance".to_string(),
            err.to_string(),
        )),
    }
}

/// get_connection establish a connection with github
pub async fn get_connection(access_token_url: &str) -> Result<Octocrab, SBError> {
    let github_key = get_key_from_env("GITHUB_KEY")?;
    let github_id = get_key_from_env("GITHUB_ID")?;

    let app_id = github_id.parse::<u64>().unwrap().into();
    let key = jsonwebtoken::EncodingKey::from_rsa_pem(github_key.as_bytes()).unwrap();
    let token = octocrab::auth::create_jwt(app_id, &key).unwrap();
    let gh = Octocrab::builder().personal_token(token).build().unwrap();

    let access_token = CreateInstallationAccessToken::default();

    let access: models::InstallationToken = gh
        .post(access_token_url, Some(&access_token))
        .await
        .unwrap();
    Ok(octocrab::OctocrabBuilder::new()
        .personal_token(access.token)
        .build()
        .unwrap())
}
