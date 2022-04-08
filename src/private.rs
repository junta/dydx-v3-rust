pub use super::structs;
use super::Result;
use chrono::{Date, DateTime, Local, Utc};
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;
use sha2::Sha256;
use std::time::Duration;

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;
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

    pub async fn get_account(&self, ethereum_address: &str) -> Result<structs::AccountsResponse> {
        let accont_id = "4b16cbab-f003-5354-bab7-e654a333d302";
        let path = format!("accounts/{}", accont_id);
        let response = self.get(path.as_str(), Vec::new()).await?;
        Ok(response)
    }

    // pub async fn get_accounts(&self) -> Result<()> {
    //     let path = "accounts";
    //     let response = self.get(path, Vec::new()).await;
    //     Ok(())
    // }

    pub async fn get<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        parameters: Vec<(&str, &str)>,
    ) -> Result<T> {
        let request_path = format!("/v3/{}", &path);
        let url = format!("{}/v3/{}", &self.host, &path);

        dbg!(&request_path);

        let iso_timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

        let signature = self.sign(request_path.as_str(), "GET", &iso_timestamp);

        let req_builder = self
            .client
            .get(url)
            .header("DYDX-SIGNATURE", signature.as_str())
            .header("DYDX-TIMESTAMP", iso_timestamp.as_str())
            .header("DYDX-API-KEY", "95ded6c7-4342-54c2-96f3-7867f3aa4dd1")
            // .header("DYDX-TIMESTAMP", "2022-04-07T08:56:13.947Z")
            .header("DYDX-PASSPHRASE", "oQmxIL1eu6rRFfeODZKt")
            .query(&parameters);

        // println!("{:?}", req_builder);
        let result = req_builder.send().await?.json::<T>().await?;
        // let res = req_builder.send().await?;
        // dbg!("{:?}", &res);

        // println!("{:?}", res.text().await);

        // let result = res.json::<T>().await?;
        Ok(result)
    }

    pub fn sign(&self, request_path: &str, method: &str, iso_timestamp: &String) -> String {
        let message = String::from(iso_timestamp) + method + request_path;
        // let message = String::from(
        //     "2022-04-07T08:56:13.947ZGET/v3/accounts/4b16cbab-f003-5354-bab7-e654a333d302",
        // );
        dbg!(&message);
        let secret = "FHKDt6oPOpE47y7gPsB6u2aMdA-rlICGhhUiC6Un";

        let secret = base64::decode_config(secret, base64::URL_SAFE).unwrap();
        // dbg!(&secret);

        let mut mac = Hmac::<Sha256>::new_from_slice(&*secret).unwrap();
        mac.update(message.as_bytes());
        // println!("{:?}", mac);
        let code = mac.finalize().into_bytes();
        let signature = base64::encode(&code);
        // dbg!(&hashed);
        return signature;
    }
}
