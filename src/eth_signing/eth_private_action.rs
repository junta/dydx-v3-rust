use super::helpers::*;

const EIP712_ETH_PRIVATE_ACTION_STRUCT_STRING_STRING: &str =
    "dYdX(string method,string requestPath,string body,string timestamp)";

const EIP712_STRUCT_NAME: &str = "dYdX";

pub struct SignEthPrivateAction<'a> {
    pub network_id: &'a str,
}

impl SignEthPrivateAction<'_> {
    // pub fn sign(signer: String, message: String) -> String {}

    pub fn getHash(&self, message: &str) -> String {
        let hashed = hash_string(message);
        dbg!(&hashed);
        hashed
    }
}
