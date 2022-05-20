macro_rules! b {
        ($e:expr) => {
                tokio_test::block_on($e)
        };
}

use dydx_v3_rust::helper::*;
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
                                stark_private_key: None,
                                web3: None,
                                eth_private_key: None
                        };
                        DydxClient::new("https://api.dydx.exchange", options)
                    }

                it "getClient" {
                        // dbg!(DydxClient().host);
                        dbg!(DydxClient().network_id);
                }
                it "getMarket" {
                        b!(async {
                                let response = DydxClient().public.get_markets(Some(DydxMarket::BTC_USD)).await.unwrap();
                                dbg!(&response.markets["BTC-USD"].oracle_price);
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

                it "getFastWithdrawal" {
                        b!(async {
                                let response = DydxClient().public.get_fast_withdrawal(None, None, None).await.unwrap();
                                dbg!(&response["liquidityProviders"]);
                        });
                }

                it "getMarketStats" {
                        b!(async {
                                let response = DydxClient().public.get_stats(DydxMarket::UNI_USD, Some(MarketStatisticDay::SEVEN)).await.unwrap();
                                dbg!(&response.markets["UNI-USD"]);
                        });
                }

                it "getHistoricalFunding" {
                        b!(async {
                                let response = DydxClient().public.get_historical_funding(DydxMarket::SUSHI_USD, None).await.unwrap();
                                dbg!(&response.historical_funding.into_iter().nth(0));
                        });
                }


                it "getCandles" {
                        b!(async {
                                let response = DydxClient().public.get_candles(DydxMarket::ETH_USD, Some(CandleResolution::FIVE_MINS), Some("2022-01-05T17:33:43.163Z"),Some("2022-01-06T17:33:43.163Z"), Some("4")).await.unwrap();
                                dbg!(response);
                        });
                }

                it "getCandlesWithNoParameter" {
                        b!(async {
                                let response = DydxClient().public.get_candles(DydxMarket::ETH_USD, None, None, None, None).await.unwrap();
                                // dbg!(response);
                        });
                }

                it "getConfig" {
                        b!(async {
                                let response = DydxClient().public.get_config().await.unwrap();
                                dbg!(response);
                        });
                }

                it "checkIfUserExists" {
                        b!(async {
                                let response = DydxClient().public.check_if_user_exists("0x72Be8d8d7d1d10d0e7f12Df508bB29b33cFFA06B").await.unwrap();
                                dbg!(response);
                        });
                }

                it "checkIfUsernameExists" {
                        b!(async {
                                let response = DydxClient().public.check_if_username_exists("faeravca").await.unwrap();
                                dbg!(response);
                        });
                }

                it "getTime" {
                        b!(async {
                                let response = DydxClient().public.get_time().await.unwrap();
                                dbg!(response);
                        });
                }

                it "getLeaderboardPnls" {
                        b!(async {
                                let response = DydxClient().public.get_leaderboard_pnls("SILVER", "2022-04-05T17:33:43.163Z", "PERCENT", None).await.unwrap();
                                dbg!(response);
                        });
                }

                it "getPublicRetroactiveMiningRewards" {
                        b!(async {
                                let response = DydxClient().public.get_public_retroactive_mining_rewards("0x72Be8d8d7d1d10d0e7f12Df508bB29b33cFFA06B").await.unwrap();
                                dbg!(response);
                        });
                }

                it "getCurrentlyRevealedHedgies" {
                        b!(async {
                                let response = DydxClient().public.get_currently_revealed_hedgies().await.unwrap();
                                dbg!(response);
                        });
                }

                it "getHistoricallyRevealedHedgies" {
                        b!(async {
                                let response = DydxClient().public.get_historically_revealed_hedgies(NftRevealType::Day, None, None).await.unwrap();
                                dbg!(response);
                        });
                }

                it "getInsuranceFundBalance" {
                        b!(async {
                                let response = DydxClient().public.get_insurance_fund_balance().await.unwrap();
                                dbg!(response);
                        });
                }

                it "getPublicProfile" {
                        b!(async {
                                let response = DydxClient().public.get_profile("SIFTBRXH").await.unwrap();
                                dbg!(response);
                        });
                }

                it "verifyEmail" {
                        b!(async {
                                let response = DydxClient().public.verify_email("aaa").await.unwrap();
                                // dbg!(response);
                        });
                }
        }
}
