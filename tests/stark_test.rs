use dydx_v3_rust::constants::*;
use dydx_v3_rust::modules::stark_sign::*;
use speculate::speculate;

#[cfg(test)]
speculate! {
        describe "starkTest" {
                it "signOrder" {
                        let sig = sign_order(TESTNET_NETWORK_ID, "ETH-USD", "BUY", "62392","145.0005","350.00067","0.125","This is an ID that the client came up with to describe this order", 1600316155, TEST_STARK_PRIVATE_KEY).unwrap();
                        println!("{}", sig);
                }
        }

}
