use super::super::helper::*;
pub use super::super::structs::*;
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

pub struct Private<'a> {
    client: reqwest::Client,
    host: &'a str,
    network_id: usize,
    api_key_credentials: ApiKeyCredentials,
    stark_private_key: Option<&'a str>,
}

impl Private<'_> {
    pub fn new<'a>(
        host: &'a str,
        network_id: usize,
        api_key_credentials: ApiKeyCredentials,
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
        let response = self
            .request(path.as_str(), Method::GET, Vec::new(), json!({}))
            .await;
        response
    }

    pub async fn get_accounts(&self) -> Result<AccountsResponse> {
        let path = "accounts";
        let response = self.request(path, Method::GET, Vec::new(), json!({})).await;
        response
    }

    pub async fn update_user(&self, data: UserParams<'_>) -> Result<UserResponse> {
        let path = "users";
        let response = self.request(path, Method::PUT, Vec::new(), data).await;
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
        let response = self.request(path, Method::GET, parameters, json!({})).await;
        response
    }

    pub async fn create_order(&self, params: ApiOrder, position_id: &str) -> Result<OrderResponse> {
        let mut parameters = params.clone();
        // let client_id = match params.client_id {
        //     Some(v) => v,
        //     None => generate_random_client_id(),
        // };
        let client_id = String::from("4038711184032522");

        let signature: String;
        match params.signature {
            Some(v) => signature = v,
            None => {
                // if let None = self.stark_private_key {
                //     return Err(Error::NoStarkKeyError);
                // }
                // TODO: sign code
                signature = String::from("02c757ea45bf5232a1960335bb73e1d246d8205d3c7c12ddec392243667e199102d4ea763bf140fe7d2abb34cc8c44c67e2804761d2e5764d36a9a37546dc49f");
            }
        }
        parameters.client_id = Some(client_id);
        parameters.signature = Some(signature);

        let path = "orders";
        // let path = format!("{}?{}", path, new_params);
        // dbg!(&path);
        let response = self
            .request(path, Method::POST, Vec::new(), parameters)
            .await;
        response
    }

    // TODO: set parameter properly
    pub async fn cancel_all_orders(&self, market: Option<&str>) -> Result<CancelOrderResponse> {
        let path = "orders";
        let mut parameters = Vec::new();
        if let Some(local_var) = market {
            parameters.push(("market", local_var));
        }
        let response = self
            .request(path, Method::DELETE, parameters, json!({}))
            .await;
        response
    }

    async fn request<T: for<'de> Deserialize<'de>, V: Serialize>(
        &self,
        path: &str,
        method: Method,
        parameters: Vec<(&str, &str)>,
        data: V,
    ) -> Result<T> {
        let request_path = if parameters.len() == 0 {
            format!("/v3/{}", &path)
        } else {
            let request_path = format!("/v3/{}", &path);
            let dummy_url = reqwest::Url::parse_with_params("https://example.net", &parameters);
            format!("{}?{}", request_path, dummy_url.unwrap().query().unwrap())
        };

        // let iso_timestamp = String::from("2022-04-12T03:29:57.239Z");
        let iso_timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

        let json = to_string(&data).unwrap();
        let json_v = to_value(&data).unwrap();

        let signature = self.sign(
            request_path.as_str(),
            method.as_str(),
            &iso_timestamp,
            Some(json.as_str()),
        );

        dbg!(&signature);

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
            .header("DYDX-API-KEY", self.api_key_credentials.key.as_str())
            .header(
                "DYDX-PASSPHRASE",
                self.api_key_credentials.passphrase.as_str(),
            )
            .query(&parameters);

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

    fn sign(
        &self,
        request_path: &str,
        method: &str,
        iso_timestamp: &String,
        data: Option<&str>,
    ) -> String {
        let mut message = String::from(iso_timestamp) + method + request_path;

        if let Some(local_var) = data {
            if local_var != "{}" {
                message.push_str(local_var);
            }
        }
        println!("{}", &message);

        let secret = self.api_key_credentials.secret.as_str();
        let secret = base64::decode_config(secret, base64::URL_SAFE).unwrap();

        let mut mac = Hmac::<Sha256>::new_from_slice(&*secret).unwrap();
        mac.update(message.as_bytes());
        let code = mac.finalize().into_bytes();
        base64::encode(&code)
    }
}
