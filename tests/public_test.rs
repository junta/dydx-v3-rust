macro_rules! b {
        ($e:expr) => {
                tokio_test::block_on($e)
        };
}

use dydx_v3_rust::structs::*;
use dydx_v3_rust::ClientOptions;
#[cfg(test)]
use dydx_v3_rust::DydxClient;

// use serde_json::json;
use speculate::speculate;

speculate! {
        describe "publicTest" {
                fn DydxClient() -> DydxClient<'static> {
                        let api_key = ApiKeyCredentials {
                                key: "ed85a071-c6b4-b4f1-c965-efb238d16c5e",
                                secret: "1iDz27dyq4RspTkP-rfTcFN6ouxTgHmTT_sKJogU",
                                passphrase: "CfbXaq6O-Yd3jKOqh10i"
                        };
                        let options: ClientOptions = ClientOptions {
                                network_id: Some(3),
                                api_timeout: None,
                                api_key_credentials: Some(api_key),
                        };
                        DydxClient::new("https://api.dydx.exchange", Some(options))
                        // DydxClient::new("https://api.dydx.exchange", None);
                    }

                it "get client" {
                        dbg!(DydxClient().host);
                        dbg!(DydxClient().network_id);
                        dbg!(DydxClient().api_key_credentials);
                        // dbg!(DydxClient().private.get_account());
                }
                it "get market test" {
                        b!(async {
                                let response = DydxClient().public.get_markets(Some(DydxMarket::BTC_USD)).await.unwrap();
                                // dbg!(response.markets.btc_usd.unwrap().max_position_size);
                        });
                }

                it "get market test with no parameter" {
                        b!(async {
                                let response = DydxClient().public.get_markets(None).await.unwrap();
                                // dbg!(response.markets.eth_usd.unwrap());
                                // dbg!(response.markets.inch_usd.unwrap());
                                // dbg!(response);
                        });
                }


                it "get orderbook test" {
                        b!(async {
                                let response = DydxClient().public.get_orderbook(DydxMarket::ETH_USD).await.unwrap();
                                println!("{:?}", response.asks[0].size);
                                // dbg!(response);
                        });
                }

                it "get trades test" {
                        b!(async {
                                let response = DydxClient().public.get_trades(DydxMarket::ETH_USD, None).await.unwrap();
                                // dbg!(response);
                        });
                }


                it "get candles test" {
                        b!(async {
                                let response = DydxClient().public.get_candles(DydxMarket::ETH_USD, Some("15MINS")).await.unwrap();
                                // println!("{:?}", response);
                                dbg!(response);
                        });
                }
        }
}
