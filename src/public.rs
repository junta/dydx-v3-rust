use super::{ResponseError, Result};
use http::StatusCode;
use serde::Deserialize;
use std::time::Duration;

pub use super::structs;

pub struct Public<'a> {
    client: reqwest::Client,
    pub host: &'a str,
}

impl Public<'_> {
    pub fn new(host: &str) -> Public {
        Public {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Client::new()"),
            host,
        }
    }

    pub async fn get_markets(&self, market: Option<&str>) -> Result<structs::MarketsResponse> {
        let mut parameter = Vec::new();
        if let Some(local_var) = market {
            parameter.push(("market", local_var));
        }
        let response: structs::MarketsResponse = self.get("markets", parameter).await?;
        Ok(response)
    }

    pub async fn get_orderbook(&self, market: &str) -> Result<structs::OrderbookResponse> {
        let path = format!("orderbook/{}", market);
        let response: structs::OrderbookResponse = self.get(path.as_str(), Vec::new()).await?;
        Ok(response)
    }

    pub async fn get_trades(
        &self,
        market: &str,
        starting_before_or_at: Option<&str>,
    ) -> Result<structs::TradesResponse> {
        let path = format!("trades/{}", market);
        let mut parameter = Vec::new();
        if let Some(local_var) = starting_before_or_at {
            parameter.push(("startingBeforeOrAt", local_var));
        }

        let response: structs::TradesResponse = self.get(path.as_str(), parameter).await?;
        Ok(response)
    }

    pub async fn get_candles(
        &self,
        market: &str,
        resolution: Option<&str>,
        from_iso: Option<&str>,
        to_iso: Option<&str>,
        // TODO: should be usize
        limit: Option<&str>,
    ) -> Result<structs::CandlesResponse> {
        let path = format!("candles/{}", market);
        let mut parameters = Vec::new();
        if let Some(local_var) = resolution {
            parameters.push(("resolution", local_var));
        }
        if let Some(local_var) = from_iso {
            parameters.push(("fromISO", local_var));
        }
        if let Some(local_var) = to_iso {
            parameters.push(("toISO", local_var));
        }
        if let Some(local_var) = limit {
            parameters.push(("limit", local_var));
        }

        let response: structs::CandlesResponse = self.get(path.as_str(), parameters).await?;
        Ok(response)
    }

    pub async fn verify_email(&self, token: &str) -> Result<StatusCode> {
        let param = vec![("token", token)];
        let response = self.put("emails/verify-email", &param).await?;
        Ok(response)
    }

    pub async fn get<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        parameters: Vec<(&str, &str)>,
    ) -> Result<T> {
        let url = format!("{}/v3/{}", &self.host, path);
        let req_builder = self.client.get(url).query(&parameters);
        // println!("{:?}", req_builder);
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

    pub async fn put(&self, path: &str, parameters: &[(&str, &str)]) -> Result<StatusCode> {
        let url = format!("{}/v3/{}", &self.host, path);
        let req_builder = self.client.put(url).query(parameters);
        let result = req_builder.send().await?;
        Ok(result.status())
    }
}
