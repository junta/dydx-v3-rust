use dydx_v3_rust::modules::stark_sign::*;

#[cfg(test)]
use speculate::speculate;

speculate! {
        describe "starkTest" {
                it "signOrder" {
                        let private_key = "58c7d5a90b1776bde86ebac077e053ed85b0f7164f53b080304a531947f46e3";
                        let sig = sign_order(3, "ETH-USD", "BUY", "62392","145.0005","350.00067","0.125","This is an ID that the client came up with to describe this order", 1600316155, private_key).unwrap();
                        println!("{}", sig);
                }
        }

}
