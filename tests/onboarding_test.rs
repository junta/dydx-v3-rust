macro_rules! b {
        ($e:expr) => {
                tokio_test::block_on($e)
        };
}

use dydx_v3_rust::constants::*;
use dydx_v3_rust::modules::eth_sign::*;
use dydx_v3_rust::ClientOptions;
use dydx_v3_rust::DydxClient;
use speculate::speculate;

#[cfg(test)]
speculate! {
        describe "onboardingTest" {
                fn DydxClient() -> DydxClient<'static> {
                        let options = ClientOptions {
                                network_id: Some(3),
                                // network_id: Some(1),
                                api_timeout: None,
                                api_key_credentials: None,
                                stark_private_key: None,
                                eth_private_key: Some(TEST_PRIVATE_KEY),
                        };
                        // DydxClient::new("https://api.dydx.exchange", options)
                        DydxClient::new("https://api.stage.dydx.exchange", options)

                }

                it "createUser" {
                        b!(async {
                                let user_data = CreateUserParams {
                                        stark_key: "05cfae6e84fa1fccf366cb2a8731407d66b1c91d77fefe767d11e70ad4b8847f",
                                        stark_key_y_coordinate: "07c2b78bedfb9ca974e0c506a7f5e0970c266c2c8992d6f29ea7ab7b1554a1a8",
                                        referred_by_affiliate_link: None,
                                        country: None

                                };

                                let _response = DydxClient().onboarding.unwrap().create_user(user_data ,TEST_ADDRESS).await;
                                // dbg!(&_response);

                        });
                }
                it "deriveStarkKey" {
                        b!(async {
                                let _response = DydxClient().onboarding.unwrap().derive_stark_key(TEST_ADDRESS);
                                // dbg!(&_response);

                        });
                }
                it "recoverDefaultApiCredentials" {
                        b!(async {
                                let onboarding = DydxClient().onboarding.unwrap();
                                let _response = onboarding.recover_default_api_key_credentials(TEST_ADDRESS);
                                // dbg!(&_response);

                        });
                }

        }

}
