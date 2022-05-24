use super::super::helper::*;
use super::super::types::*;
use super::super::{ResponseError, Result};
use super::stark_sign::*;
use chrono::prelude::*;
use hmac::{Hmac, Mac};
use http::{Method, StatusCode};
use reqwest::RequestBuilder;
use serde::Deserialize;
use serde::Serialize;
use serde_json::*;
use sha2::Sha256;
use std::time::Duration;

#[derive(Debug, Clone)]
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

    pub async fn get_api_keys(&self) -> Result<ApiKeysResponse> {
        let response = self
            .request("api-keys", Method::GET, Vec::new(), json!({}))
            .await;
        response
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
        let response = self
            .request("accounts", Method::GET, Vec::new(), json!({}))
            .await;
        response
    }

    pub async fn update_user(&self, data: UserParams<'_>) -> Result<UserResponse> {
        let response = self.request("users", Method::PUT, Vec::new(), data).await;
        response
    }

    pub async fn create_account(
        &self,
        stark_key: &str,
        stark_key_y_coordinate: &str,
    ) -> Result<AccountResponse> {
        let data = CreateAccountParams {
            stark_key,
            stark_key_y_coordinate,
        };

        let response = self
            .request("accounts", Method::POST, Vec::new(), data)
            .await;
        response
    }

    pub async fn get_positions(
        &self,
        market: Option<&str>,
        status: Option<&str>,
        limit: Option<&str>,
        created_before_or_at: Option<&str>,
    ) -> Result<PositionsResponse> {
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
        let response = self
            .request("positions", Method::GET, parameters, json!({}))
            .await;
        response
    }

    pub async fn create_order(&self, user_params: ApiOrderParams<'_>) -> Result<OrderResponse> {
        let client_id = generate_random_client_id();

        let signature = sign_order(
            self.network_id,
            user_params.market,
            user_params.side,
            user_params.position_id,
            user_params.size,
            user_params.price,
            user_params.limit_fee,
            &client_id,
            user_params.expiration,
            self.stark_private_key.unwrap(),
            user_params.path,
        )
        .unwrap();

        let naive = NaiveDateTime::from_timestamp(user_params.expiration, 0);
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        let expiration_second = datetime.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

        let parameters = ApiOrder {
            market: user_params.market,
            side: user_params.side,
            type_field: user_params.type_field,
            size: user_params.size,
            price: user_params.price,
            time_in_force: user_params.time_in_force,
            post_only: user_params.post_only,
            limit_fee: user_params.limit_fee,
            cancel_id: user_params.cancel_id,
            trigger_price: user_params.trigger_price,
            trailing_percent: user_params.trailing_percent,
            expiration: expiration_second.as_str(),
            client_id: client_id.as_str(),
            signature: signature.as_str(),
        };

        let response = self
            .request("orders", Method::POST, Vec::new(), parameters)
            .await;
        response
    }

    pub async fn create_withdraw(
        &self,
        user_params: ApiWithdrawParams<'_>,
    ) -> Result<WithdrawalResponse> {
        let client_id = generate_random_client_id();

        let signature = sign_withdraw(
            self.network_id,
            user_params.position_id,
            user_params.amount,
            &client_id,
            user_params.expiration,
            self.stark_private_key.unwrap(),
            user_params.path,
        )
        .unwrap();

        let naive = NaiveDateTime::from_timestamp(user_params.expiration, 0);
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        let expiration_second = datetime.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

        let parameters = ApiWithdraw {
            amount: user_params.amount,
            asset: user_params.asset,
            expiration: expiration_second.as_str(),
            client_id: client_id.as_str(),
            signature: signature.as_str(),
        };

        let response = self
            .request("withdrawals", Method::POST, Vec::new(), parameters)
            .await;
        response
    }

    pub async fn get_transfers(
        &self,
        transfer_type: &str,
        limit: Option<&str>,
        created_before_or_at: Option<&str>,
    ) -> Result<TransfersResponse> {
        let mut parameters = vec![("transferType", transfer_type)];
        if let Some(local_var) = limit {
            parameters.push(("limit", local_var));
        }
        if let Some(local_var) = created_before_or_at {
            parameters.push(("createdBeforeOrAt", local_var));
        }
        let response = self
            .request("transfers", Method::GET, Vec::new(), parameters)
            .await;
        response
    }

    pub async fn cancel_all_orders(&self, market: Option<&str>) -> Result<CancelOrderResponse> {
        let mut parameters = Vec::new();
        if let Some(local_var) = market {
            parameters.push(("market", local_var));
        }
        let response = self
            .request("orders", Method::DELETE, parameters, json!({}))
            .await;
        response
    }

    pub async fn send_verification_email(&self) -> Result<StatusCode> {
        let response = self.put("emails/send-verification-email").await;
        response
    }

    pub async fn request_testnet_tokens(&self) -> Result<TransferResponse> {
        let response = self
            .request("testnet/tokens", Method::POST, Vec::new(), json!({}))
            .await;
        response
    }

    pub async fn get_profile(&self) -> Result<ProfilePrivateResponse> {
        let response = self
            .request("profile/private", Method::GET, Vec::new(), json!({}))
            .await;
        response
    }

    async fn put(&self, path: &str) -> Result<StatusCode> {
        let url = format!("{}/v3/{}", &self.host, path);
        let req_builder = self.client.put(url);
        let result = req_builder.send().await?;
        Ok(result.status())
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

        let iso_timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
        let json = to_string(&data).unwrap();

        let signature = self.sign(
            request_path.as_str(),
            method.as_str(),
            &iso_timestamp,
            Some(json.as_str()),
        );

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

        let req_builder = if json != "{}" {
            req_builder.json(&data)
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
        // println!("{}", &message);

        let secret = self.api_key_credentials.secret;
        let secret = base64::decode_config(secret, base64::URL_SAFE).unwrap();

        let mut mac = Hmac::<Sha256>::new_from_slice(&*secret).unwrap();
        mac.update(message.as_bytes());
        let code = mac.finalize().into_bytes();
        base64::encode(&code)
    }
}
