macro_rules! b {
        ($e:expr) => {
                tokio_test::block_on($e)
        };
}

use dotenv::dotenv;
use dydx_v3_rust::helper::*;
use dydx_v3_rust::structs::*;
use dydx_v3_rust::ClientOptions;
#[cfg(test)]
use dydx_v3_rust::DydxClient;
use std::env;

// use serde_json::json;
use speculate::speculate;

speculate! {
        describe "ethPrivateTest" {
                async fn DydxClient() -> DydxClient<'static> {
                        let transport = web3::transports::Http::new("https://mainnet.infura.io/v3/ce7426bf07f24fd59a2f7bbb6df217b4").unwrap();
                        let web3 = web3::Web3::new(transport);
                        let eth_private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
                        let options = ClientOptions {
                                network_id: Some(3),
                                api_timeout: None,
                                api_key_credentials: None,
                                stark_private_key: None,
                                web3: Some(web3),
                                eth_private_key: Some(eth_private_key),
                        };
                        // DydxClient::new("https://api.dydx.exchange", Some(options))
                        DydxClient::new("https://api.stage.dydx.exchange", options)
                }

                it "getEthPrivate" {
                        b!(async {
                                let transport = web3::transports::Http::new("https://mainnet.infura.io/v3/ce7426bf07f24fd59a2f7bbb6df217b4").unwrap();
                                let web3 = web3::Web3::new(transport);
                                let mut accounts = web3.eth().accounts().await;
                                println!("Accounts: {:?}", accounts);

                                accounts.unwrap().push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());

                        });
                }
        }

}
