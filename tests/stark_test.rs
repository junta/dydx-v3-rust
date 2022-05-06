use dotenv::dotenv;
use dydx_v3_rust::helper::*;
use dydx_v3_rust::stark_sign::*;
use dydx_v3_rust::structs::*;
use dydx_v3_rust::structs::*;
use dydx_v3_rust::ClientOptions;

#[cfg(test)]
// use serde_json::json;
use speculate::speculate;

speculate! {
        describe "starkTest" {
                it "signOrder" {
                        let sig = sign_order().unwrap();
                        println!("{}", sig);
                }
        }

}
