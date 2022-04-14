use super::super::helper::*;
pub use super::super::structs::*;
use super::super::{ResponseError, Result};
use chrono::Utc;
use hmac::{Hmac, Mac};
use http::{Method, StatusCode};
use reqwest::RequestBuilder;
use serde::Deserialize;
use serde::Serialize;
use serde_json::*;
use sha2::Sha256;
use std::time::Duration;
use web3::transports::Http;
use web3::Transport;
use web3::Web3;

pub struct EthPrivate<'a> {
    client: reqwest::Client,
    host: &'a str,
    network_id: usize,
    // TODO: should accept IPC, WS
    web3: web3::Web3<Http>,
    sec_key: secp256k1::SecretKey,
}

impl EthPrivate<'_> {
    pub fn new<'a>(
        host: &'a str,
        network_id: usize,
        web3: web3::Web3<Http>,
        eth_private_key: String,
    ) -> EthPrivate<'a> {
        EthPrivate {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Client::new()"),
            host,
            network_id,
            web3,
            sec_key: eth_private_key.parse().unwrap(),
        }
    }
}
