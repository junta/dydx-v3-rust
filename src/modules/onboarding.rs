pub use super::super::types::*;
use super::super::{ResponseError, Result};
use super::eth_sign::*;
use chrono::Utc;
use http::{Method, StatusCode};
use serde::Deserialize;
use serde::Serialize;
use serde_json::*;

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Onboarding<'a> {
    client: reqwest::Client,
    host: &'a str,
    network_id: usize,
    eth_private_key: &'a str,
}

impl Onboarding<'_> {
    pub fn new<'a>(host: &'a str, network_id: usize, eth_private_key: &'a str) -> Onboarding<'a> {
        Onboarding {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(30))
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

    // pub fn derive_stark_key(&self, ethereum_address: &str) -> Result<KeyPairWithYCoordinate> {
    //     let action = "dYdX STARK Key";
    //     let signature = sign_onboarding(
    //         self.network_id,
    //         ethereum_address,
    //         action,
    //         self.eth_private_key,
    //     )
    //     .unwrap();
    //     response
    // }

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
                    // println!("{}", response.text().await.unwrap());
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
