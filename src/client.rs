use super::{Api, Error, Result};
use http::StatusCode;
use reqwest::RequestBuilder;
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

impl Client {
    pub fn new(host: Option<&str>) -> Client {
        Client {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Client::new()"),
            api: if let Some(host) = host {
                Api::Production
            } else {
                Api::Staging
            },
        }
    }

    pub async fn markets(&self, parameters: &Value) -> Result<Response> {
        self.get("markets", parameters).await
    }

    pub async fn get(&self, endpoint: &str, parameters: &Value) -> Result<Response> {
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
