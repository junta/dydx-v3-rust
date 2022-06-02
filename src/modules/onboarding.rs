pub use super::super::types::*;
use super::super::{ResponseError, Result};
use super::eth_sign::*;
use http::StatusCode;
use serde::Deserialize;
use serde::Serialize;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Onboarding<'a> {
    client: reqwest::Client,
    host: &'a str,
    network_id: usize,
    eth_private_key: &'a str,
}

impl Onboarding<'_> {
    pub fn new<'a>(host: &'a str, network_id: usize, api_timeout: u64, eth_private_key: &'a str) -> Onboarding<'a> {
        Onboarding {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(api_timeout))
                .build()
                .expect("Client::new()"),
            host,
            network_id,
            eth_private_key,
        }
    }

    pub async fn create_user(
        &self,
        params: CreateUserParams<'_>,
        ethereum_address: &str,
    ) -> Result<CreateUserResponse> {
        let path = "onboarding";
        let response = self.request(path, ethereum_address, params).await;
        response
    }

    pub fn derive_stark_key(&self, ethereum_address: &str) -> Result<String> {
        let action = "dYdX STARK Key";
        let signature = sign_onboarding(
            self.network_id,
            ethereum_address,
            action,
            self.eth_private_key,
        )
        .unwrap();
        let stark_private_key = derive_stark_private_key(signature).unwrap();
        Ok(stark_private_key)
    }

    pub fn recover_default_api_key_credentials(
        &self,
        ethereum_address: &str,
    ) -> Result<ApiKeyCredentialsResponseObject> {
        let action = "dYdX Onboarding";
        let signature = sign_onboarding(
            self.network_id,
            ethereum_address,
            action,
            self.eth_private_key,
        )
        .unwrap();
        let sig_str = signature.as_str();
        let r_hex = &sig_str[2..66];

        let secret = derive_secret(r_hex.to_string()).unwrap();
        let s_hex = &sig_str[66..130];
        let passphrase = derive_passphrase(s_hex.to_string()).unwrap();
        let key = derive_key(s_hex.to_string()).unwrap();

        let api_key = ApiKeyCredentialsResponseObject {
            key: key,
            secret: secret,
            passphrase: passphrase,
        };
        Ok(api_key)
    }

    async fn request<T: for<'de> Deserialize<'de>, V: Serialize>(
        &self,
        path: &str,
        ethereum_address: &str,
        data: V,
    ) -> Result<T> {
        let action = "dYdX Onboarding";

        let signature = sign_onboarding(
            self.network_id,
            ethereum_address,
            action,
            self.eth_private_key,
        )
        .unwrap();

        let url = format!("{}/v3/{}", &self.host, path);

        let req_builder = self.client.post(url);

        let req_builder = req_builder
            .header("DYDX-SIGNATURE", signature.as_str())
            .header("DYDX-ETHEREUM-ADDRESS", ethereum_address)
            .json(&data);

        let response = req_builder.send().await;

        match response {
            Ok(response) => match response.status() {
                StatusCode::OK | StatusCode::CREATED => {
                    return Ok(response.json::<T>().await.unwrap())
                }
                _ => {
                    let error = ResponseError {
                        code: response.status().to_string(),
                        message: response.text().await.unwrap(),
                    };
                    return Err(Box::new(error));
                }
            },
            Err(err) => {
                return Err(Box::new(err));
            }
        };
    }
}
