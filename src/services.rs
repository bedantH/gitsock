use std::env;
use tokio::time::sleep;
use std::time::Duration;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::Deserialize;
use crate::crypto::encrypt;

const GITHUB_AUTH_BASE_URL: &'static str = "https://github.com";
const GITHUB_API_BASE_URL: &'static str = "https://api.github.com";

static GITHUB_CLIENT_SECRET: Lazy<String> = Lazy::new(|| {
    env::var("GITHUB_OAUTH_CLIENT_SECRET").expect("GITHUB_CLIENT_SECRET")
});
static GITHUB_CLIENT_ID: Lazy<String> = Lazy::new(|| {
    env::var("GITHUB_OAUTH_CLIENT_ID").expect("GITHUB_CLIENT_ID")
});

static client: Lazy<Client> = Lazy::new(|| {
    reqwest::Client::new()
});

#[derive(Deserialize, Debug)]
pub(crate) struct DeviceCodeResponse {
    pub(crate) device_code: String,
    pub(crate) user_code: String,
    pub(crate) verification_uri: String,
    pub(crate) expires_in: u64,
    pub(crate) interval: u64,
}

#[derive(Deserialize, Debug)]
struct AccessTokenResponse {
    access_token: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

pub async fn start_device_login_flow() -> Result<DeviceCodeResponse, Box<dyn std::error::Error>> {
    let params = [
        ("client_id", GITHUB_CLIENT_ID.clone()),
        ("scope", String::from("user, read:public_key")),
    ];

    let res = client.post(format!("{}/login/device/code", GITHUB_AUTH_BASE_URL))
        .header("Accept", "application/vnd.github.v3+json")
        .form(&params)
        .send()
        .await.unwrap();
    
    let status = res.status();

    if !res.status().is_success() {
        let text = res.text().await?;
        return Err(format!("Device code request failed: HTTP {}: {}", status, text).into());
    }

    let device_code_response = res.json::<DeviceCodeResponse>().await?;
    Ok(device_code_response)
}


pub async fn poll_for_token(device_code: String, interval: u64) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let params = [
        ("client_id", GITHUB_CLIENT_ID.clone()),
        ("client_secret", GITHUB_CLIENT_SECRET.clone()),
        ("device_code", device_code),
        ("grant_type", "urn:ietf:params:oauth:grant-type:device_code".parse().unwrap()),
    ];
    let mut auth_token: Option<String> = None;

    loop {
        sleep(Duration::from_secs(interval)).await;

        let res = client.post(format!("{}/login/oauth/access_token", GITHUB_AUTH_BASE_URL))
            .header("Accept", "application/vnd.github.v3+json")
            .form(&params)
            .send()
            .await?;

        let status = res.status();

        if !res.status().is_success() {
            let text = res.text().await?;
            eprintln!("Token poll failed: HTTP {}: {}", status, text);
            break;
        }

        let data = res.json::<AccessTokenResponse>().await?;

        if let Some(token) = data.access_token {
            auth_token = Some(token);
            break;
        } else if data.error.as_deref() != Some("authorization_pending") {
            eprintln!("❌ Error: {:?}", data.error_description);
            break;
        }
    }

    Ok(auth_token)
}

#[derive(Deserialize, Debug)]
pub(crate) struct UserInfoResponse {
    pub(crate) login: String,
    pub(crate) email: Option<String>,
    pub(crate) name: String,
}

pub async fn get_user_info(token: String) -> Result<UserInfoResponse, Box<dyn std::error::Error>> {
    let res = client.get(format!("{}/user", GITHUB_API_BASE_URL))
        .header("Accept", "application/vnd.github.v3+json")
        .header("Authorization", format!("Bearer {}", token))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "GitSock")
        .send()
        .await?;

    let status = res.status();
    let text = res.text().await?;

    if !status.is_success() {
        return Err(format!("User info request failed: HTTP {}: {}", status, text).into());
    }

    let user_info: UserInfoResponse = serde_json::from_str(&text)
        .map_err(|e| format!("Failed to parse user info: {}. Body: {}", e, text))?;

    Ok(user_info)
}