macro_rules! b {
        ($e:expr) => {
                tokio_test::block_on($e)
        };
}

use dydx_v3_rust::constants::*;
use dydx_v3_rust::types::*;
use dydx_v3_rust::ClientOptions;
use dydx_v3_rust::DydxClient;
use speculate::speculate;

#[cfg(test)]
speculate! {
        describe "publicTest" {
                fn DydxClient() -> DydxClient<'static> {

                        let options: ClientOptions = ClientOptions {
                                network_id: None,
                                api_timeout: None,
                                api_key_credentials: None,
                                stark_private_key: None,
                                eth_private_key: None
                        };
                        DydxClient::new("https://api.dydx.exchange", options)
                    }

                it "getMarket" {
                        b!(async {
                                let _response = DydxClient().public.get_markets(Some(DydxMarket::BTC_USD)).await.unwrap();
                                // dbg!(&_response.markets["BTC-USD"].oracle_price);
                        });
                }

                it "getMarketWithNoParameter" {
                        b!(async {
                                let _response = DydxClient().public.get_markets(None).await.unwrap();
                                // dbg!(&_response.markets["1INCH-USD"]);
                                // dbg!(_response);
                        });
                }


                it "getOrderbook" {
                        b!(async {
                                let _response = DydxClient().public.get_orderbook(DydxMarket::ETH_USD).await.unwrap();
                                // println!("{:?}", _response.asks[0].size);
                                // dbg!(_response);
                        });
                }

                it "getTrades" {
                        b!(async {
                                let _response = DydxClient().public.get_trades(DydxMarket::ETH_USD, None).await.unwrap();
                                // dbg!(_response);
                        });
                }

                it "getFastWithdrawal" {
                        b!(async {
                                let _response = DydxClient().public.get_fast_withdrawal(None, None, None).await.unwrap();
                                // dbg!(&_response["liquidityProviders"]);
                        });
                }

                it "getMarketStats" {
                        b!(async {
                                let _response = DydxClient().public.get_stats(DydxMarket::UNI_USD, Some(MarketStatisticDay::SEVEN)).await.unwrap();
                                // dbg!(&_response.markets["UNI-USD"]);
                        });
                }

                it "getHistoricalFunding" {
                        b!(async {
                                let _response = DydxClient().public.get_historical_funding(DydxMarket::SUSHI_USD, None).await.unwrap();
                                // dbg!(&_response.historical_funding.into_iter().nth(0));
                        });
                }


                it "getCandles" {
                        b!(async {
                                let _response = DydxClient().public.get_candles(DydxMarket::ETH_USD, Some(CandleResolution::FIVE_MINS), Some("2022-01-05T17:33:43.163Z"),Some("2022-01-06T17:33:43.163Z"), Some("4")).await.unwrap();
                                // dbg!(_response);
                        });
                }

                it "getCandlesWithNoParameter" {
                        b!(async {
                                let _response = DydxClient().public.get_candles(DydxMarket::ETH_USD, None, None, None, None).await.unwrap();
                                // dbg!(_response);
                        });
                }

                it "getConfig" {
                        b!(async {
                                let _response = DydxClient().public.get_config().await.unwrap();
                                // dbg!(_response);
                        });
                }

                it "checkIfUserExists" {
                        b!(async {
                                let _response = DydxClient().public.check_if_user_exists(TEST_ADDRESS).await.unwrap();
                                // dbg!(_response);
                        });
                }

                it "checkIfUsernameExists" {
                        b!(async {
                                let _response = DydxClient().public.check_if_username_exists("faeravca").await.unwrap();
                                // dbg!(_response);
                        });
                }

                it "getTime" {
                        b!(async {
                                let _response = DydxClient().public.get_time().await.unwrap();
                                // dbg!(_response);
                        });
                }

                it "getLeaderboardPnls" {
                        b!(async {
                                let _response = DydxClient().public.get_leaderboard_pnls("SILVER", "2022-04-05T17:33:43.163Z", "PERCENT", None).await.unwrap();
                                // dbg!(_response);
                        });
                }

                it "getPublicRetroactiveMiningRewards" {
                        b!(async {
                                let _response = DydxClient().public.get_public_retroactive_mining_rewards(TEST_ADDRESS).await.unwrap();
                                // dbg!(_response);
                        });
                }

                it "getCurrentlyRevealedHedgies" {
                        b!(async {
                                let _response = DydxClient().public.get_currently_revealed_hedgies().await.unwrap();
                                // dbg!(_response);
                        });
                }

                it "getHistoricallyRevealedHedgies" {
                        b!(async {
                                let _response = DydxClient().public.get_historically_revealed_hedgies(NftRevealType::DAY, None, None).await.unwrap();
                                // dbg!(_response);
                        });
                }

                it "getInsuranceFundBalance" {
                        b!(async {
                                let _response = DydxClient().public.get_insurance_fund_balance().await.unwrap();
                                // dbg!(_response);
                        });
                }

                it "getPublicProfile" {
                        b!(async {
                                let _response = DydxClient().public.get_profile("SIFTBRXH").await.unwrap();
                                // dbg!(_response);
                        });
                }

                it "verifyEmail" {
                        b!(async {
                                let _response = DydxClient().public.verify_email("aaa").await.unwrap();
                                // dbg!(_response);
                        });
                }
        }
}
