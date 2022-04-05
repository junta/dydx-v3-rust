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

    pub fn get_account(&self, ethereum_address: &str) -> usize {
        1
    }
}
