macro_rules! b {
        ($e:expr) => {
                tokio_test::block_on($e)
        };
}
#[cfg(test)]
use dydx_v3_rust::Client;

use serde_json::json;
use speculate::speculate;

speculate! {
        describe "publicTest" {
                fn client() -> Client {
                        Client::new("production")
                    }
                it "get market test" {
                        b!(async {
                                let response = client().get_markets(&json!({"market": "BTC-USD"})).await.unwrap();
                                // let body = response.json().await.unwrap();
                                dbg!(response["markets"]["BTC-USD"]["volume24H"].as_str().unwrap());
                                // dbg!(response);
                        });
                }


                it "get orderbook test" {
                        b!(async {
                                let response = client().get_orderbook().await.unwrap();
                                // let body = response.json().await.unwrap();
                                // println!("{:?}", response);
                                dbg!(response);
                        });
                }
        }
}
