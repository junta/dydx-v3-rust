macro_rules! b {
        ($e:expr) => {
                tokio_test::block_on($e)
        };
}

use dydx_v3_rust::constants::*;
use dydx_v3_rust::ClientOptions;
use dydx_v3_rust::DydxClient;
use speculate::speculate;

#[cfg(test)]
speculate! {
        describe "ethPrivateTest" {
                fn DydxClient() -> DydxClient<'static> {
                        let options = ClientOptions {
                                network_id: Some(TESTNET_NETWORK_ID),
                                api_timeout: None,
                                api_key_credentials: None,
                                stark_private_key: None,
                                eth_private_key: Some(TEST_PRIVATE_KEY),
                        };
                        DydxClient::new(TESTNET_API_URL, options)

                }


                it "recovery" {
                        b!(async {
                                let _response = DydxClient().eth_private.unwrap().recovery(TEST_ADDRESS).await.unwrap();
                                // dbg!(&_response);
                        });
                }

                it "createAndDeleteApiKey" {
                        b!(async {
                                let _response = DydxClient().eth_private.unwrap().create_api_key(TEST_ADDRESS).await.unwrap();
                                // dbg!(&_response);
                                let _delete__response = DydxClient().eth_private.unwrap().delete_api_key(_response.api_key.key.as_str(), TEST_ADDRESS).await;
                                // dbg!(&_delete__response);
                        });
                }
        }

}
