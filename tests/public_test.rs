#[cfg(test)]
extern crate speculate;

use dydx_v3_rust::public;

use speculate::speculate;

speculate! {
        describe "publicTest" {
                it "get market test" {
                        public::get_markets();
                }
        }
}
