macro_rules! b {
        ($e:expr) => {
                tokio_test::block_on($e)
        };
}
#[cfg(test)]
use dydx_v3_rust::DydxClient;

// use serde_json::json;
use speculate::speculate;

speculate! {
        describe "publicTest" {
                fn DydxClient() -> DydxClient<'static> {
                        DydxClient::new("https://api.dydx.exchange", None)
                    }

                it "get client" {
                        dbg!(DydxClient().host);
                }
                it "get market test" {
                        b!(async {
                                let response = DydxClient().public.get_markets(Some(&vec![("market", "BTC-USD")])).await.unwrap();
                                dbg!(response.markets.btc_usd.unwrap().max_position_size);
                        });
                }

                it "get market test with no parameter" {
                        b!(async {
                                let response = DydxClient().public.get_markets(None).await.unwrap();
                                dbg!(response.markets.eth_usd.unwrap());
                                // dbg!(response.markets.inch_usd.unwrap());
                                // dbg!(response);
                        });
                }


                it "get orderbook test" {
                        b!(async {
                                let response = DydxClient().public.get_orderbook().await.unwrap();
                                // println!("{:?}", response.asks[0].size);
                                // dbg!(response);
                        });
                }
        }
}
