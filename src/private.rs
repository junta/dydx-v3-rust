use super::helper::*;
pub use super::structs::*;
use super::{Error, Result};
use chrono::Utc;
use hmac::{Hmac, Mac};
use http::{Method, StatusCode};
use serde::Deserialize;
use sha2::Sha256;
use std::time::Duration;

pub struct Private<'a> {
    client: reqwest::Client,
    host: &'a str,
    network_id: usize,
    api_key_credentials: ApiKeyCredentials<'a>,
    stark_private_key: Option<&'a str>,
}

impl Private<'_> {
    pub fn new<'a>(
        host: &'a str,
        network_id: usize,
        api_key_credentials: ApiKeyCredentials<'a>,
        stark_private_key: Option<&'a str>,
    ) -> Private<'a> {
        Private {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Client::new()"),
            host,
            network_id,
            api_key_credentials,
            stark_private_key,
        }
    }

    pub async fn get_account(&self, ethereum_address: &str) -> Result<AccountResponse> {
        let account_id = get_account_id(ethereum_address);
        let path = format!("accounts/{}", account_id);
        let response = self.request(path.as_str(), Method::GET, Vec::new()).await;
        response
    }

    pub async fn get_accounts(&self) -> Result<AccountsResponse> {
        let path = "accounts";
        let response = self.request(path, Method::GET, Vec::new()).await;
        response
    }

    pub async fn create_order(
        &self,
        params: ApiOrder,
        position_id: &str,
    ) -> Result<AccountsResponse> {
        let client_id = match params.client_id {
            Some(v) => v,
            None => generate_random_client_id(),
        };

        let signature: String;
        match params.signature {
            Some(v) => signature = v,
            None => {
                if let None = self.stark_private_key {
                    return Err(Error::NoStarkKeyError);
                }
                // TODO: sign code
                signature = String::from("aaaa");
            }
        }
        let mut parameters = params.clone();
        parameters.client_id = Some(client_id);
        parameters.signature = Some(signature);

        let path = "orders";
        let response = self.request(path, Method::POST, parameters).await;
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
            parameters.push(("createdBeforeOrAt", local_var));
        }
        let response = self.request(path, Method::GET, parameters).await;
        response
    }

    pub async fn request<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        method: Method,
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

        let url = format!("{}/v3/{}", &self.host, path);

        let req_builder = match method {
            Method::GET => self.client.get(url),
            Method::POST => self.client.post(url),
            Method::PUT => self.client.put(url),
            Method::DELETE => self.client.delete(url),
            _ => self.client.get(url),
        };

        let req_builder = req_builder
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
