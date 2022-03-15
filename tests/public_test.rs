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
                                let response = client().get_markets(&json!({})).await.unwrap();
                                dbg!(response);
                        });
                }
        }
}
