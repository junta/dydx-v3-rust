macro_rules! b {
        ($e:expr) => {
                tokio_test::block_on($e)
        };
}

use dydx_v3_rust::db::*;
use dydx_v3_rust::structs::*;
use dydx_v3_rust::ClientOptions;
#[cfg(test)]
use dydx_v3_rust::DydxClient;

// use serde_json::json;
use speculate::speculate;

speculate! {
        describe "publicTest" {
                fn DydxClient() -> DydxClient<'static> {

                        let options: ClientOptions = ClientOptions {
                                network_id: None,
                                api_timeout: None,
                                api_key_credentials: None,
                        };
                        DydxClient::new("https://api.dydx.exchange", options)
                    }

                it "getClient" {
                        // dbg!(DydxClient().host);
                        dbg!(DydxClient().network_id);
                        dbg!(DydxClient().api_key_credentials);
                        // dbg!(DydxClient().private.get_account("0x1e88f23864a8FE784eB152967AccDb394D3b88AD"));
                }
                it "getMarket" {
                        b!(async {
                                let response = DydxClient().public.get_markets(Some(DydxMarket::BTC_USD)).await.unwrap();
                                // dbg!(response.markets.btc_usd.unwrap().max_position_size);
                        });
                }

                it "getMarketWithNoParameter" {
                        b!(async {
                                let response = DydxClient().public.get_markets(None).await.unwrap();
                                // dbg!(response.markets.eth_usd.unwrap());
                                // dbg!(response.markets.inch_usd.unwrap());
                                // dbg!(response);
                        });
                }


                it "getOrderbook" {
                        b!(async {
                                let response = DydxClient().public.get_orderbook(DydxMarket::ETH_USD).await.unwrap();
                                // println!("{:?}", response.asks[0].size);
                                // dbg!(response);
                        });
                }

                it "getTrades" {
                        b!(async {
                                let response = DydxClient().public.get_trades(DydxMarket::ETH_USD, None).await.unwrap();
                                // dbg!(response);
                        });
                }


                it "getCandles" {
                        b!(async {
                                let response = DydxClient().public.get_candles(DydxMarket::ETH_USD, Some("15MINS"), Some("2022-01-05T17:33:43.163Z"),Some("2022-01-06T17:33:43.163Z"), Some("4")).await.unwrap();
                                // println!("{:?}", response);
                                // dbg!(response);
                        });
                }

                it "getCandlesWithNoParameter" {
                        b!(async {
                                let response = DydxClient().public.get_candles(DydxMarket::ETH_USD, None, None, None, None).await.unwrap();
                                // println!("{:?}", response);
                                // dbg!(response);
                        });
                }


                it "verifyEmail" {
                        b!(async {
                                let response = DydxClient().public.verify_email("aaa").await.unwrap();
                                // println!("{:?}", response);
                                // dbg!(response);
                        });
                }
        }
}
