pub use super::structs;
use super::Result;
use serde::Deserialize;
use std::time::Duration;
pub struct Private<'a> {
    client: reqwest::Client,
    host: &'a str,
    network_id: usize,
    api_key_credentials: structs::ApiKeyCredentials<'a>,
}

impl Private<'_> {
    pub fn new<'a>(
        host: &'a str,
        network_id: usize,
        api_key_credentials: structs::ApiKeyCredentials<'a>,
    ) -> Private<'a> {
        Private {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Client::new()"),
            host,
            network_id,
            api_key_credentials,
        }
    }

    pub async fn get_account(&self, ethereum_address: &str) -> Result<structs::AccountResponse> {
        let accont_id = "ae00878c-b6a9-52bc-abf6-25f24219fd4a";
        let path = format!("accounts/{}", accont_id);
        let param = vec![("ethereumAddress", ethereum_address)];
        let response: structs::AccountResponse = self.get(path.as_str(), param).await?;
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
}
