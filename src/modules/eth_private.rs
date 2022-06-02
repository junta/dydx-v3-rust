pub use super::super::types::*;
use super::super::{ResponseError, Result};
use super::eth_sign::*;
use chrono::Utc;
use http::{Method, StatusCode};
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct EthPrivate<'a> {
    client: reqwest::Client,
    host: &'a str,
    network_id: usize,
    eth_private_key: &'a str,
}

impl EthPrivate<'_> {
    pub fn new<'a>(host: &'a str, network_id: usize, api_timeout: u64, eth_private_key: &'a str) -> EthPrivate<'a> {
        EthPrivate {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(api_timeout))
                .build()
                .expect("Client::new()"),
            host,
            network_id,
            eth_private_key,
        }
    }

    pub async fn recovery(&self, ethereum_address: &str) -> Result<RecoveryResponse> {
        let response = self
            .request("recovery", Method::GET, ethereum_address)
            .await;
        response
    }

    pub async fn create_api_key(
        &self,
        ethereum_address: &str,
    ) -> Result<ApiKeyCredentialsResponse> {
        let response = self
            .request("api-keys", Method::POST, ethereum_address)
            .await;
        response
    }

    pub async fn delete_api_key(
        &self,
        api_key: &str,
        ethereum_address: &str,
    ) -> Result<StatusCode> {
        let parameter = vec![("apiKey", api_key)];
        let response = self.delete("api-keys", ethereum_address, parameter).await;
        response
    }

    async fn request<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        method: Method,
        ethereum_address: &str,
    ) -> Result<T> {
        let iso_timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

        let request_path = format!("/v3/{}", path);

        let signature = sign_private(
            self.network_id,
            ethereum_address,
            method.as_str(),
            request_path.as_str(),
            "{}",
            &iso_timestamp,
            self.eth_private_key,
        )
        .unwrap();

        let url = format!("{}/v3/{}", &self.host, path);

        let req_builder = match method {
            Method::GET => self.client.get(url),
            Method::POST => self.client.post(url),
            _ => self.client.get(url),
        };

        let req_builder = req_builder
            .header("DYDX-SIGNATURE", signature.as_str())
            .header("DYDX-TIMESTAMP", iso_timestamp.as_str())
            .header("DYDX-ETHEREUM-ADDRESS", ethereum_address);

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

    async fn delete(
        &self,
        path: &str,
        ethereum_address: &str,
        parameters: Vec<(&str, &str)>,
    ) -> Result<StatusCode> {
        let request_path = if parameters.len() == 0 {
            format!("/v3/{}", &path)
        } else {
            let request_path = format!("/v3/{}", &path);
            let dummy_url = reqwest::Url::parse_with_params("https://example.net", &parameters);
            format!("{}?{}", request_path, dummy_url.unwrap().query().unwrap())
        };
        let iso_timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

        let signature = sign_private(
            self.network_id,
            ethereum_address,
            "DELETE",
            request_path.as_str(),
            "{}",
            &iso_timestamp,
            self.eth_private_key,
        )
        .unwrap();

        let url = format!("{}/v3/{}", &self.host, path);

        let req_builder = self.client.delete(url);

        let req_builder = req_builder
            .header("DYDX-SIGNATURE", signature.as_str())
            .header("DYDX-TIMESTAMP", iso_timestamp.as_str())
            .header("DYDX-ETHEREUM-ADDRESS", ethereum_address)
            .query(&parameters);

        let response = req_builder.send().await;

        Ok(response.unwrap().status())
    }
}
