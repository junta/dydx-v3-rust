use super::{Api, Error, Result};
use http::StatusCode;
use reqwest::RequestBuilder;
use serde::Deserialize;
use serde_json::Value;
use std::time::Duration;

pub struct Client {
    client: reqwest::Client,
    api: Api,
}

#[derive(Debug)]
pub struct Response {
    pub response: reqwest::Response,
    pub request: reqwest::Request,
}

impl Response {
    pub async fn json(self) -> Result<Value> {
        Ok(self.response.json().await?)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketsResponse {
    pub markets: Markets,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Markets {
    #[serde(rename = "BTC-USD")]
    pub btc_usd: MarketData,
    #[serde(rename = "SUSHI-USD")]
    pub sushi_usd: MarketData,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketData {
    pub market: String,
    pub status: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub step_size: String,
    pub tick_size: String,
    pub index_price: String,
    pub oracle_price: String,
    #[serde(rename = "priceChange24H")]
    pub price_change24h: String,
    pub next_funding_rate: String,
    pub next_funding_at: String,
    pub min_order_size: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub initial_margin_fraction: String,
    pub maintenance_margin_fraction: String,
    #[serde(rename = "volume24H")]
    pub volume24h: String,
    #[serde(rename = "trades24H")]
    pub trades24h: String,
    pub open_interest: String,
    pub incremental_initial_margin_fraction: String,
    pub incremental_position_size: String,
    pub max_position_size: String,
    pub baseline_position_size: String,
    pub asset_resolution: String,
    pub synthetic_asset_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookResponse {
    pub asks: Vec<OrderbookResponseOrder>,
    pub bids: Vec<OrderbookResponseOrder>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookResponseOrder {
    pub size: String,
    pub price: String,
}

impl Client {
    pub fn new(env: &str) -> Client {
        Client {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Client::new()"),
            api: if env == "production" {
                Api::Production
            } else {
                Api::Staging
            },
        }
    }

    pub async fn get_markets(&self, parameters: &Value) -> Result<Value> {
        let response = self.get_withparam("markets", parameters).await?;
        let body = response.json().await.unwrap();
        // let deserialized: Markets = serde_json::from_str(&body).unwrap();

        Ok(body)
    }

    // pub async fn get_orderbook(&self) -> Result<Value> {
    //     let response = self.get("orderbook/BTC-USD").await?;
    //     let body = response.json().await.unwrap();

    //     Ok(body)
    // }

    pub async fn get_orderbook(&self) -> Result<OrderbookResponse> {
        let response: OrderbookResponse = self.call_json("orderbook/BTC-USD").await?;
        // let body = response.json().await.unwrap();

        Ok(response)
    }
    // pub async fn get_trades(&self, parameters: &Value) -> Result<Response> {
    //     self.get("trades", parameters).await
    // }

    pub async fn call_json<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<T> {
        let url = format!("{}/{}", &self.api.url(), path);
        let result = self.client.get(url).send().await?.json::<T>().await?;
        Ok(result)
    }

    pub async fn get(&self, endpoint: &str) -> Result<Response> {
        let request = self.client.get(format!("{}/{}", self.api.url(), endpoint));
        Ok(self.request(request).await?)
    }

    pub async fn get_withparam(&self, endpoint: &str, parameters: &Value) -> Result<Response> {
        let request = self
            .client
            .get(format!("{}/{}", self.api.url(), endpoint))
            .query(parameters);
        Ok(self.request(request).await?)
    }

    async fn request(&self, request: RequestBuilder) -> Result<Response> {
        let request = request.build()?;

        let response = self
            .client
            .execute(request.try_clone().expect(
                "Error can remain unhandled because we're not using streams, which are the try_clone fail condition",
            ))
            .await;

        match &response {
            Ok(response) => match response.status() {
                StatusCode::NOT_FOUND => return Err(Error::NotFoundError),
                StatusCode::UNAUTHORIZED => return Err(Error::AuthenticationError),
                StatusCode::BAD_REQUEST => return Err(Error::InvalidRequestError),
                _ => {}
            },
            Err(err) => {
                if err.is_connect() || err.is_timeout() {
                    return Err(Error::ApiConnectionError);
                }
            }
        };

        Ok(Response {
            response: response?,
            request,
        })
    }
}
