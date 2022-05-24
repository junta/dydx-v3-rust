macro_rules! b {
        ($e:expr) => {
                tokio_test::block_on($e)
        };
}
use dydx_v3_rust::helper::*;
use dydx_v3_rust::types::*;
use dydx_v3_rust::ClientOptions;
#[cfg(test)]
use dydx_v3_rust::DydxClient;
use speculate::speculate;
use std::str::FromStr;

speculate! {
        describe "onboardingTest" {
                fn DydxClient() -> DydxClient<'static> {
                        let options = ClientOptions {
                                network_id: Some(3),
                                api_timeout: None,
                                api_key_credentials: None,
                                stark_private_key: None,
                                eth_private_key: Some("2d06e246aaac1458ca0712e3faac6cacd2ed35bda0853998a80568948a3e3b46"),
                        };
                        // DydxClient::new("https://api.dydx.exchange", Some(options))
                        DydxClient::new("https://api.stage.dydx.exchange", options)

                }

                // it "getOnboarding" {
                //         b!(async {
                //                 let sign = dydx_v3_rust::OnboardingAction {
                //                         network_id: "1"
                //                 };

                //                 // dbg!(signature);
                //         });
                // }
                it "createUser" {
                        b!(async {
                                let userData = CreateUserParams {
                                        stark_key:"0474df88a6f75dcce483a85d1cdd60034c820736a44cde52005610844fa2d2c7",
                                        stark_key_y_coordinate: "0276f9cc8377c22e272ab3147fc7bdc339ce087259bcfdd153c4d5f356783a4f",
                                        referred_by_affiliate_link: None,
                                        country: None

                                };

                                let response = DydxClient().onboarding.unwrap().create_user(userData ,"0x1e88f23864a8FE784eB152967AccDb394D3b88AD").await;
                                dbg!(&response);

                        });
                }

        }

}
