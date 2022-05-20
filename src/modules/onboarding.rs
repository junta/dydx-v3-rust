use super::super::helper::*;
pub use super::super::types::*;
use super::super::{ResponseError, Result};
use chrono::Utc;
use hmac::{Hmac, Mac};
use http::{Method, StatusCode};
use reqwest::RequestBuilder;
use serde::Deserialize;
use serde::Serialize;
use serde_json::*;
use sha2::Sha256;
use std::time::Duration;
use web3::transports::Http;
use web3::Transport;
use web3::Web3;
pub struct Onboarding<'a> {
    client: reqwest::Client,
    host: &'a str,
    network_id: usize,
}

impl Onboarding<'_> {
    pub fn new<'a>(
        host: &'a str,
        network_id: usize,
        // web3: web3::Web3<Http>,
        // eth_private_key: String,
    ) -> Onboarding<'a> {
        Onboarding {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Client::new()"),
            host,
            network_id,
            // web3,
            // sec_key: eth_private_key.parse().unwrap(),
        }
    }

    pub async fn create_user(
        &self,
        params: CreateUserParams<'_>,
        ethereum_address: &str,
    ) -> Result<ApiKeyCredentialsResponse> {
        let path = "onboarding";
        let response = self.request(path, ethereum_address, params).await;
        response
    }

    async fn request<T: for<'de> Deserialize<'de>, V: Serialize>(
        &self,
        path: &str,
        ethereum_address: &str,
        data: V,
    ) -> Result<T> {
        let json_v = to_value(&data).unwrap();

        // let signature = self.sign(
        //     request_path.as_str(),
        //     method.as_str(),
        //     &iso_timestamp,
        //     Some(json.as_str()),
        // );

        let signature = String::from("0x2eb4c584446a01b332f608712d0f3193cf10332bc78367479c707bdd1e15f8b565c3b9507421c86a1ac2e5dd03a6e19f3676293c0d2db3b52252bdc529a6e7d61b01");

        dbg!(&signature);

        let url = format!("{}/v3/{}", &self.host, path);

        let req_builder = self.client.post(url);

        let req_builder = req_builder
            .header("DYDX-SIGNATURE", signature.as_str())
            .header("DYDX-ETHEREUM-ADDRESS", ethereum_address)
            .json(&json_v);

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
