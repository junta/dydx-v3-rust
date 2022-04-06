use super::Result;
use http::StatusCode;
use reqwest::RequestBuilder;
use serde::Deserialize;
use serde_json::Value;
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
        let mut parameter = Vec::new();
        if let Some(local_var) = resolution {
            parameter.push(("resolution", local_var));
        }
        if let Some(local_var) = from_iso {
            parameter.push(("fromISO", local_var));
        }
        if let Some(local_var) = to_iso {
            parameter.push(("toISO", local_var));
        }
        if let Some(local_var) = limit {
            parameter.push(("limit", local_var));
        }

        let response: structs::CandlesResponse = self.get(path.as_str(), parameter).await?;
        Ok(response)
    }

    pub async fn verify_email(&self, token: &str) -> Result<http::StatusCode> {
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
        let result = req_builder.send().await?.json::<T>().await?;
        Ok(result)
    }

    pub async fn put(
        &self,
        path: &str,
        parameters: &Vec<(&str, &str)>,
    ) -> Result<http::StatusCode> {
        let url = format!("{}/v3/{}", &self.host, path);
        let req_builder = self.client.put(url).query(parameters);
        let result = req_builder.send().await?;
        Ok(result.status())
    }

    // #[derive(Debug)]
    // pub struct Response {
    //     pub response: reqwest::Response,
    //     pub request: reqwest::Request,
    // }

    // impl Response {
    //     pub async fn json(self) -> Result<Value> {
    //         Ok(self.response.json().await?)
    //     }
    // }

    // pub async fn get(&self, endpoint: &str) -> Result<Response> {
    //     let request = self.client.get(format!("{}/{}", self.api.url(), endpoint));
    //     Ok(self.request(request).await?)
    // }

    // pub async fn get_withparam(&self, endpoint: &str, parameters: &Value) -> Result<Response> {
    //     let request = self
    //         .client
    //         .get(format!("{}/{}", self.api.url(), endpoint))
    //         .query(parameters);
    //     Ok(self.request(request).await?)
    // }

    // async fn request(&self, request: RequestBuilder) -> Result<Response> {
    //     let request = request.build()?;

    //     let response = self
    //         .client
    //         .execute(request.try_clone().expect(
    //             "Error can remain unhandled because we're not using streams, which are the try_clone fail condition",
    //         ))
    //         .await;

    //     match &response {
    //         Ok(response) => match response.status() {
    //             StatusCode::NOT_FOUND => return Err(Error::NotFoundError),
    //             StatusCode::UNAUTHORIZED => return Err(Error::AuthenticationError),
    //             StatusCode::BAD_REQUEST => return Err(Error::InvalidRequestError),
    //             _ => {}
    //         },
    //         Err(err) => {
    //             if err.is_connect() || err.is_timeout() {
    //                 return Err(Error::ApiConnectionError);
    //             }
    //         }
    //     };

    //     Ok(Response {
    //         response: response?,
    //         request,
    //     })
    // }
}
