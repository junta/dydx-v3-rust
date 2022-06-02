use super::super::helper::*;
use super::super::types::*;
use super::super::{ResponseError, Result};
use super::stark_sign::*;
use chrono::prelude::*;
use hmac::{Hmac, Mac};
use http::{Method, StatusCode};
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
        api_timeout: u64,
        api_key_credentials: ApiKeyCredentials<'a>,
        stark_private_key: Option<&'a str>,
    ) -> Private<'a> {
        Private {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(api_timeout))
                .build()
                .expect("Client::new()"),
            host,
            network_id,
            api_key_credentials,
            stark_private_key,
        }
    }

    pub async fn get_registration(&self) -> Result<RegistrationResponse> {
        let response = self
            .request("registration", Method::GET, Vec::new(), json!({}))
            .await;
        response
    }

    pub async fn get_user(&self) -> Result<UserResponse> {
        let response = self
            .request("users", Method::GET, Vec::new(), json!({}))
            .await;
        response
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

    pub async fn get_account_leaderboard_pnl(
        &self,
        period: &str,
        starting_before_or_at: Option<&str>,
    ) -> Result<AccountPnlsResponse> {
        let path = format!("accounts/leaderboard-pnl/{}", period);

        let mut parameters = Vec::new();
        if let Some(local_var) = starting_before_or_at {
            parameters.push(("startingBeforeOrAt", local_var));
        }

        let response = self
            .request(path.as_str(), Method::GET, parameters, json!({}))
            .await;
        response
    }

    pub async fn get_historical_leaderboard_pnls(
        &self,
        period: &str,
        limit: Option<&str>,
    ) -> Result<HistoricalLeaderboardPnlsResponse> {
        let path = format!("accounts/historical-leaderboard-pnls/{}", period);

        let mut parameters = Vec::new();
        if let Some(local_var) = limit {
            parameters.push(("limit", local_var));
        }

        let response = self
            .request(path.as_str(), Method::GET, parameters, json!({}))
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

    pub async fn create_fast_withdraw(
        &self,
        user_params: ApiFastWithdrawalParams<'_>,
    ) -> Result<WithdrawalResponse> {
        // let client_id = generate_random_client_id();
        let client_id = "8113483921639613";
        // let nonce = nonce_from_client_id(&client_id);
        // let fact = get_transfer_erc20_fact(
        //     user_params.to_address,
        //     6,
        //     user_params.credit_amount,
        //     "0x8707A5bf4C2842d46B31A405Ba41b858C0F876c4",
        //     nonce,
        // )
        // .unwrap();

        // dbg!(&fact.as_str());

        let signature = sign_fast_withdraw(
            self.network_id,
            user_params.position_id,
            user_params.lp_position_id,
            user_params.lp_stark_key,
            "0x8Fb814935f7E63DEB304B500180e19dF5167B50e",
            user_params.to_address,
            6,
            user_params.credit_amount,
            "0x8707A5bf4C2842d46B31A405Ba41b858C0F876c4",
            &client_id,
            user_params.expiration,
            self.stark_private_key.unwrap(),
        )
        .unwrap();
        dbg!(&signature);

        let naive = NaiveDateTime::from_timestamp(user_params.expiration, 0);
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        let expiration_second = datetime.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

        let parameters = ApiFastWithdrawal {
            credit_asset: user_params.credit_asset,
            credit_amount: user_params.credit_amount,
            debit_amount: user_params.debit_amount,
            to_address: user_params.to_address,
            lp_position_id: user_params.lp_position_id,
            expiration: expiration_second.as_str(),
            client_id: client_id,
            signature: signature.as_str(),
        };

        let response = self
            .request("fast-withdrawals", Method::POST, Vec::new(), parameters)
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

    pub async fn cancel_order(&self, order_id: &str) -> Result<CancelOrderResponse> {
        let path = format!("orders/{}", order_id);
        let response = self
            .request(path.as_str(), Method::DELETE, Vec::new(), json!({}))
            .await;
        response
    }

    pub async fn cancel_all_orders(&self, market: Option<&str>) -> Result<CancelOrdersResponse> {
        let mut parameters = Vec::new();
        if let Some(local_var) = market {
            parameters.push(("market", local_var));
        }
        let response = self
            .request("orders", Method::DELETE, parameters, json!({}))
            .await;
        response
    }

    pub async fn get_orders(
        &self,
        market: Option<&str>,
        status: Option<&str>,
        side: Option<&str>,
        type_field: Option<&str>,
        limit: Option<&str>,
        created_before_or_at: Option<&str>,
        return_latest_orders: Option<&str>,
    ) -> Result<OrdersResponse> {
        let mut parameters = Vec::new();
        if let Some(local_var) = market {
            parameters.push(("market", local_var));
        }
        if let Some(local_var) = status {
            parameters.push(("status", local_var));
        }
        if let Some(local_var) = side {
            parameters.push(("side", local_var));
        }
        if let Some(local_var) = type_field {
            parameters.push(("type", local_var));
        }
        if let Some(local_var) = limit {
            parameters.push(("limit", local_var));
        }
        if let Some(local_var) = created_before_or_at {
            parameters.push(("created_before_or_at", local_var));
        }
        if let Some(local_var) = return_latest_orders {
            parameters.push(("return_latest_orders", local_var));
        }
        let response = self
            .request("orders", Method::GET, Vec::new(), json!({}))
            .await;
        response
    }

    pub async fn get_active_orders(
        &self,
        market: &str,
        side: Option<&str>,
        id: Option<&str>,
    ) -> Result<ActiveOrdersResponse> {
        let mut parameters = vec![("market", market)];
        if let Some(local_var) = side {
            parameters.push(("side", local_var));
        }
        if let Some(local_var) = id {
            parameters.push(("id", local_var));
        }
        let response = self
            .request("active-orders", Method::GET, parameters, json!({}))
            .await;
        response
    }

    pub async fn get_order_by_id(&self, id: &str) -> Result<OrderResponse> {
        let path = format!("orders/{}", id);
        let response = self
            .request(path.as_str(), Method::GET, Vec::new(), json!({}))
            .await;
        response
    }

    pub async fn get_order_by_client_id(&self, id: &str) -> Result<OrderResponse> {
        let path = format!("orders/client/{}", id);
        let response = self
            .request(path.as_str(), Method::GET, Vec::new(), json!({}))
            .await;
        response
    }

    pub async fn get_fills(
        &self,
        market: Option<&str>,
        order_id: Option<&str>,
        limit: Option<&str>,
        created_before_or_at: Option<&str>,
    ) -> Result<FillsResponse> {
        let mut parameters = Vec::new();
        if let Some(local_var) = market {
            parameters.push(("market", local_var));
        }
        if let Some(local_var) = order_id {
            parameters.push(("orderId", local_var));
        }
        if let Some(local_var) = limit {
            parameters.push(("limit", local_var));
        }
        if let Some(local_var) = created_before_or_at {
            parameters.push(("createdBeforeOrAt", local_var));
        }
        let response = self
            .request("fills", Method::GET, parameters, json!({}))
            .await;
        response
    }

    pub async fn get_funding_payments(
        &self,
        market: Option<&str>,
        limit: Option<&str>,
        effective_before_or_at: Option<&str>,
    ) -> Result<FundingResponse> {
        let mut parameters = Vec::new();
        if let Some(local_var) = market {
            parameters.push(("market", local_var));
        }
        if let Some(local_var) = limit {
            parameters.push(("limit", local_var));
        }
        if let Some(local_var) = effective_before_or_at {
            parameters.push(("effectiveBeforeOrAt", local_var));
        }
        let response = self
            .request("funding", Method::GET, parameters, json!({}))
            .await;
        response
    }

    pub async fn get_historical_pnl(
        &self,
        effective_before_or_at: Option<&str>,
        effective_at_or_after: Option<&str>,
    ) -> Result<HistoricalPnlResponse> {
        let mut parameters = Vec::new();
        if let Some(local_var) = effective_before_or_at {
            parameters.push(("effectiveBeforeOrAt", local_var));
        }
        if let Some(local_var) = effective_at_or_after {
            parameters.push(("effectiveAtOrAfter", local_var));
        }
        let response = self
            .request("historical-pnl", Method::GET, parameters, json!({}))
            .await;
        response
    }

    pub async fn get_trading_rewards(&self, epoch: Option<&str>) -> Result<TradingRewardsResponse> {
        let mut parameters = Vec::new();
        if let Some(local_var) = epoch {
            parameters.push(("epoch", local_var));
        }
        let response = self
            .request("rewards/weight", Method::GET, parameters, json!({}))
            .await;
        response
    }

    pub async fn get_liquidity_provider_rewards(
        &self,
        epoch: Option<&str>,
    ) -> Result<LiquidityProviderRewardsResponse> {
        let mut parameters = Vec::new();
        if let Some(local_var) = epoch {
            parameters.push(("epoch", local_var));
        }
        let response = self
            .request("rewards/liquidity", Method::GET, parameters, json!({}))
            .await;
        response
    }

    pub async fn get_retroactive_mining_rewards(&self) -> Result<RetroactiveMiningRewardsResponse> {
        let response = self
            .request(
                "rewards/retroactive-mining",
                Method::GET,
                Vec::new(),
                json!({}),
            )
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
