#[cfg(test)]
use dydx_v3_rust::public;

use speculate::speculate;

speculate! {
        describe "publicTest" {
                it "get market test" {
                        let result = public::get_markets("BTC-USD");
                }
        }
}
