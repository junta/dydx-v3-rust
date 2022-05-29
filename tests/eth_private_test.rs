macro_rules! b {
        ($e:expr) => {
                tokio_test::block_on($e)
        };
}

use dydx_v3_rust::ClientOptions;
use dydx_v3_rust::DydxClient;
use speculate::speculate;

#[cfg(test)]
speculate! {
        describe "ethPrivateTest" {
                fn DydxClient() -> DydxClient<'static> {
                        let test_eth_private_key = "2d06e246aaac1458ca0712e3faac6cacd2ed35bda0853998a80568948a3e3b46";
                        let options = ClientOptions {
                                network_id: Some(3),
                                api_timeout: None,
                                api_key_credentials: None,
                                stark_private_key: None,
                                eth_private_key: Some(test_eth_private_key),
                        };
                        // DydxClient::new("https://api.dydx.exchange", Some(options))
                        DydxClient::new("https://api.stage.dydx.exchange", options)

                }

                it "recovery" {
                        b!(async {
                                let address = "0x72Be8d8d7d1d10d0e7f12Df508bB29b33cFFA06B";
                                let response = DydxClient().eth_private.unwrap().recovery(address).await.unwrap();
                                dbg!(&response);
                        });
                }

                it "createAndDeleteApiKey" {
                        b!(async {
                                let address = "0x72Be8d8d7d1d10d0e7f12Df508bB29b33cFFA06B";
                                let response = DydxClient().eth_private.unwrap().create_api_key(address).await.unwrap();
                                dbg!(&response);
                                let delete_response = DydxClient().eth_private.unwrap().delete_api_key(response.api_key.key.as_str(), address).await;
                                dbg!(&delete_response);
                        });
                }
        }

}
