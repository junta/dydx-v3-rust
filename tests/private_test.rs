macro_rules! b {
        ($e:expr) => {
                tokio_test::block_on($e)
        };
}

use chrono::{DateTime, Duration, Utc};
use dydx_v3_rust::helper::*;
use dydx_v3_rust::types::*;
use dydx_v3_rust::ClientOptions;
#[cfg(test)]
use dydx_v3_rust::DydxClient;
// use serde_json::json;
use speculate::speculate;

speculate! {
        describe "privateTest" {
                fn DydxClient() -> DydxClient<'static> {
                        let api_key = ApiKeyCredentials {
                                // test
                                key: "d58be525-1ec9-5775-c435-61c12a3c4d6a",
                                secret: "cYKIG0mX0mLXyw4IG1pBUf7qx1ycnUm593pTKnQT",
                                passphrase: "-rgYL26TLDp62Aq5BTek"
                        };
                        let options = ClientOptions {
                                network_id: Some(3),
                                api_timeout: None,
                                api_key_credentials: Some(api_key),
                                stark_private_key: Some("039dd31bce09db7330e9fce956f111a5e9fc2be2e2aaecb065f05e8a42fcbca4"),
                                web3: None,
                                eth_private_key: None
                        };
                        // DydxClient::new("https://api.dydx.exchange", Some(options))
                        DydxClient::new("https://api.stage.dydx.exchange", options)
                    }

                it "getClient" {
                        // dbg!(DydxClient().host);
                        // dbg!(DydxClient().network_id);
                        // dbg!(DydxClient().api_key_credentials);
                }

                it "getAccountId" {
                        b!(async {
                                let uuid = get_account_id("0x0De1C59f3AA4938B0bDcC070B4Fa9F395A4D6d25");
                                // println!("{:?}", response);
                                // dbg!(uuid);
                        });
                }

                it "getAccount" {
                        b!(async {
                                let testAddress = "0x72Be8d8d7d1d10d0e7f12Df508bB29b33cFFA06B";
                                let response = DydxClient().private.unwrap().get_account(testAddress).await.unwrap();
                                // dbg!(response);
                        });
                }

                it "getAccountUnauthorized" {
                        b!(async {
                                fn DydxClientNonAuth() -> DydxClient<'static> {
                                        let api_key = ApiKeyCredentials {
                                                // account2 testnet
                                                key: "ed85a071-c6b4-b4f1-c965-efb238d16c5e",
                                                secret: "1iDz27dyq4RspTkP-rfTcFN6ouxTgHmTT_sKJogU",
                                                passphrase: "CfbXaq6O-Yd3jKOqh10a"
                                        };
                                        let options = ClientOptions {
                                                network_id: Some(3),
                                                api_timeout: None,
                                                api_key_credentials: Some(api_key),
                                                stark_private_key: Some("0657eaa201ba872f72c0e6e2db278d8cda1b60de4313f02213aaf2b3421bff56"),
                                                web3: None,
                                                eth_private_key: None
                                        };
                                        // DydxClient::new("https://api.dydx.exchange", Some(options))
                                        DydxClient::new("https://api.stage.dydx.exchange", options)
                                    }

                                let response = DydxClientNonAuth().private.unwrap().get_accounts().await;
                                match response {
                                        Ok(v) => println!("{:?}", v),
                                        Err(e) => println!("{:?}", e),
                                    }
                                // dbg!(response);
                        });
                }

                it "getAccounts" {
                        b!(async {
                                let response = DydxClient().private.unwrap().get_accounts().await.unwrap();
                                // dbg!(&response);
                                assert_eq!(response.accounts[0].position_id, "62683");
                        });
                }

                it "getPositionsWithNoParameters" {
                        b!(async {
                                let response = DydxClient().private.unwrap().get_positions(None, None, None, None).await.unwrap();
                                // dbg!(response);
                        });
                }

                it "getPositions" {
                        b!(async {
                                let response = DydxClient().private.unwrap().get_positions(Some(DydxMarket::BTC_USD), None, None, Some("2022-04-01T02:43:02.946Z")).await.unwrap();
                                // dbg!(response);
                        });
                }

                it "generateClientId" {
                        b!(async {
                                let client_id = generate_random_client_id();
                                // dbg!(client_id);
                        });
                }

                it "createOrder" {
                        b!(async {
                                let datetime_now: DateTime<Utc> = Utc::now();
                                let expiration = datetime_now + Duration::minutes(3);
                                let expiration_unix = expiration.timestamp();

                                let order_params = ApiOrderParams {
                                        position_id: "62392",
                                        market: DydxMarket::BTC_USD,
                                        side: OrderSide::BUY,
                                        type_field: OrderType::MARKET,
                                        time_in_force: TimeInForce::FOK,
                                        post_only: false,
                                        size: "0.01",
                                        price: "100000",
                                        limit_fee: "0.1",
                                        cancel_id: None,
                                        trigger_price: None,
                                        trailing_percent: None,
                                        expiration: expiration_unix,
                                };
                                let order = DydxClient().private.unwrap().create_order(order_params).await.unwrap();
                                dbg!(order);
                        });
                }

                it "updateUser" {
                        b!(async {
                                let userData = UserParams {
                                        email: Some("eaese@example.com"),
                                        user_data: "{}",
                                        username: None,
                                        is_sharing_username: None,
                                        is_sharing_address: Some(true),
                                        country: None

                                };
                                let user = DydxClient().private.unwrap().update_user(userData).await.unwrap();
                                dbg!(user);
                        });
                }

                it "createAccount" {
                        b!(async {
                                let test_stark_public_key = "0084868a931891833df41ff83980f79889a045a46bbd273ec60ff402d7a1293f";
                                let test_stark_public_key_y_coordinate = "024b4de622d8acc9198800f5607c4a25968425fe268afe0272dbd3d5e119407d";
                                let response = DydxClient().private.unwrap().create_account(test_stark_public_key, test_stark_public_key_y_coordinate).await.unwrap();
                                // println!("{:?}", response);
                        });
                }

                // it "sign" {
                //         b!(async {
                //                 let signed = DydxClient().private.unwrap().sign("/v3/users", "PUT", &String::from("2022-04-12T04:11:00.369Z"), Some(r#"{"email":"fff@example.com","userData":"{}"}"#));
                //                 dbg!(signed);
                //         });
                // }

                it "cancelOrders" {
                        b!(async {
                                let response = DydxClient().private.unwrap().cancel_all_orders(Some(DydxMarket::BTC_USD)).await.unwrap();
                                // println!("{:?}", response);
                        });
                }

                it "requestTestnetTokens" {
                        b!(async {
                                let response = DydxClient().private.unwrap().request_testnet_tokens().await.unwrap();
                                // println!("{:?}", response);
                        });
                }


                it "sendVerificationEmail" {
                        b!(async {
                                let response = DydxClient().private.unwrap().send_verification_email().await.unwrap();
                                // println!("{:?}", response);
                                assert_eq!(response, 400);
                        });
                }

                it "getPrivateProfile" {
                        b!(async {
                                let response = DydxClient().private.unwrap().get_profile().await.unwrap();
                                println!("{:?}", response);
                        });
                }
        }
}
