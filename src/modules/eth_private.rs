// use crate::SignEthPrivateAction;

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

pub struct EthPrivate<'a> {
    client: reqwest::Client,
    host: &'a str,
    network_id: usize,
    // TODO: should accept IPC, WS
    web3: web3::Web3<Http>,
    // sec_key: secp256k1::SecretKey,
    // signer: SignEthPrivateAction,
}

impl EthPrivate<'_> {
    pub fn new<'a>(
        host: &'a str,
        network_id: usize,
        web3: web3::Web3<Http>,
        eth_private_key: &String,
    ) -> EthPrivate<'a> {
        EthPrivate {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Client::new()"),
            host,
            network_id,
            web3,
            // signer: SignEthPrivateAction::new(network_id, eth_private_key.to_string()),
            // sec_key: eth_private_key.parse().unwrap(),
        }
    }

    pub async fn create_api_key(
        &self,
        ethereum_address: &str,
    ) -> Result<ApiKeyCredentialsResponse> {
        let path = "api-keys";
        let response = self
            .request(path, Method::POST, ethereum_address, json!({}))
            .await;
        response
    }

    pub async fn delete_api_key(
        &self,
        api_key: &str,
        ethereum_address: &str,
    ) -> Result<ApiKeyCredentialsResponse> {
        let path = "api-keys";
        let response = self
            .request(path, Method::DELETE, ethereum_address, api_key)
            .await;
        response
    }

    async fn request<T: for<'de> Deserialize<'de>, V: Serialize>(
        &self,
        path: &str,
        method: Method,
        ethereum_address: &str,
        data: V,
    ) -> Result<T> {
        // let iso_timestamp = String::from("2022-04-12T03:29:57.239Z");
        let iso_timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

        let json = to_string(&data).unwrap();
        let json_v = to_value(&data).unwrap();

        // let signature = self.sign(
        //     request_path.as_str(),
        //     method.as_str(),
        //     &iso_timestamp,
        //     Some(json.as_str()),
        // );

        // let signature = self.signer.sign();

        let signature = String::from("aaa");

        dbg!(&signature);

        let url = format!("{}/v3/{}", &self.host, path);

        let req_builder = match method {
            Method::GET => self.client.get(url),
            Method::POST => self.client.post(url),
            Method::DELETE => self.client.delete(url),
            _ => self.client.get(url),
        };

        let req_builder = req_builder
            .header("DYDX-SIGNATURE", signature.as_str())
            .header("DYDX-TIMESTAMP", iso_timestamp.as_str())
            .header("DYDX-ETHEREUM-ADDRESS", ethereum_address);

        let req_builder = if json != "{}" {
            req_builder.json(&json_v)
        } else {
            req_builder
        };
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
