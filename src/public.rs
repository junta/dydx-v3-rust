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
        let parameter = match market {
            Some(v) => {
                let mut params = Vec::new();
                params.push(("market", v));
                Some(params)
            }
            None => None,
        };
        let response: structs::MarketsResponse = self.get("markets", parameter).await?;
        Ok(response)
    }

    pub async fn get_orderbook(&self, market: &str) -> Result<structs::OrderbookResponse> {
        let path = format!("orderbook/{}", market);
        let response: structs::OrderbookResponse = self.get(path.as_str(), None).await?;
        Ok(response)
    }

    // pub async fn verify_email(&self, token: &str) -> Result<T> {
    //     let response = self.put("emails/verify-email", token).await?;
    //     Ok(response)
    // }

    pub async fn get<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        parameters: Option<Vec<(&str, &str)>>,
    ) -> Result<T> {
        let url = format!("{}/v3/{}", &self.host, path);
        let req_builder = match parameters {
            Some(v) => self.client.get(url).query(&v),
            None => self.client.get(url),
        };
        let result = req_builder.send().await?.json::<T>().await?;
        Ok(result)
    }

    pub async fn put<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        parameters: &Vec<(&str, &str)>,
    ) -> Result<T> {
        let url = format!("{}/v3/{}", &self.host, path);
        let req_builder = self.client.put(url).query(parameters);
        let result = req_builder.send().await?.json::<T>().await?;
        Ok(result)
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

    // pub async fn get_trades(&self, parameters: &Value) -> Result<Response> {
    //     self.get("trades", parameters).await
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
