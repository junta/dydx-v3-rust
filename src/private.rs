pub use super::structs::*;
use super::{Error, Result};
use chrono::Utc;
use hmac::{Hmac, Mac};
use http::StatusCode;
use serde::Deserialize;
use sha2::Sha256;
use std::time::Duration;

pub struct Private<'a> {
    client: reqwest::Client,
    host: &'a str,
    network_id: usize,
    api_key_credentials: ApiKeyCredentials<'a>,
}

impl Private<'_> {
    pub fn new<'a>(
        host: &'a str,
        network_id: usize,
        api_key_credentials: ApiKeyCredentials<'a>,
    ) -> Private<'a> {
        Private {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Client::new()"),
            host,
            network_id,
            api_key_credentials,
        }
    }

    pub async fn get_account(&self, ethereum_address: &str) -> Result<AccountResponse> {
        let accont_id = "ae00878c-b6a9-52bc-abf6-25f24219fd4a";
        let path = format!("accounts/{}", accont_id);
        // let response = self.get(path.as_str(), Vec::new()).await?;
        let response = self.get(path.as_str(), Vec::new()).await;
        response
    }

    pub async fn get_accounts(&self) -> Result<AccountsResponse> {
        let path = "accounts";
        let response = self.get(path, Vec::new()).await;
        response
    }

    pub async fn get_positions(
        &self,
        market: Option<&str>,
        status: Option<&str>,
        limit: Option<&str>,
        created_before_or_at: Option<&str>,
    ) -> Result<PositionsResponse> {
        let path = "positions";
        let mut parameters = Vec::new();
        if let Some(local_var) = market {
            parameters.push(("market", local_var));
        }
        if let Some(local_var) = status {
            parameters.push(("status", local_var));
        }
        if let Some(local_var) = limit {
            parameters.push(("limit", local_var));
        }
        if let Some(local_var) = created_before_or_at {
            parameters.push(("created_before_or_at", local_var));
        }
        let response = self.get(path, parameters).await;
        response
    }

    pub async fn get<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        parameters: Vec<(&str, &str)>,
    ) -> Result<T> {
        let request_path = if parameters.len() == 0 {
            format!("/v3/{}", &path)
        } else {
            let request_path = format!("/v3/{}", &path);
            let dummy_url = reqwest::Url::parse_with_params("https://example.net", &parameters);
            format!("{}?{}", request_path, dummy_url.unwrap().query().unwrap())
        };

        let iso_timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

        let signature = self.sign(request_path.as_str(), "GET", &iso_timestamp);

        let url = format!("{}/v3/{}", &self.host, &path);

        let req_builder = self
            .client
            .get(url)
            .header("DYDX-SIGNATURE", signature.as_str())
            .header("DYDX-TIMESTAMP", iso_timestamp.as_str())
            .header("DYDX-API-KEY", self.api_key_credentials.key)
            .header("DYDX-PASSPHRASE", self.api_key_credentials.passphrase)
            .query(&parameters);

        let response = req_builder.send().await;

        match response {
            Ok(response) => match response.status() {
                StatusCode::OK => return Ok(response.json::<T>().await?),
                StatusCode::NOT_FOUND => return Err(Error::NotFoundError),
                StatusCode::UNAUTHORIZED => return Err(Error::AuthenticationError),
                StatusCode::BAD_REQUEST => return Err(Error::InvalidRequestError),
                _ => {
                    println!("{:?}", response);
                    return Err(Error::InvalidRequestError);
                }
            },
            Err(err) => {
                // if err.is_connect() || err.is_timeout() {
                return Err(Error::ApiConnectionError);
                // }
            }
        };

        // let result = response?.json::<T>().await?;

        // Ok(result)
        // let res = req_builder.send().await?;
        // dbg!("{:?}", &res);

        // println!("{:?}", res.text().await);

        // let result = res.json::<T>().await?;
    }

    pub fn sign(&self, request_path: &str, method: &str, iso_timestamp: &String) -> String {
        let message = String::from(iso_timestamp) + method + request_path;

        let secret = self.api_key_credentials.secret;
        let secret = base64::decode_config(secret, base64::URL_SAFE).unwrap();

        let mut mac = Hmac::<Sha256>::new_from_slice(&*secret).unwrap();
        mac.update(message.as_bytes());
        let code = mac.finalize().into_bytes();
        base64::encode(&code)
    }
}
