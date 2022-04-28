use eth_encode_packed::abi;
use eth_encode_packed::ethabi::ethereum_types::U256;
use eth_encode_packed::SolidityDataType;
use keccak_hash::keccak;

const EIP712_ONBOARDING_ACTION_STRUCT_STRING: &str = "dYdX(string action,string onlySignOn)";

const EIP712_ONBOARDING_ACTION_STRUCT_STRING_TESTNET: &str =
    "dYdX(string action,string onlySignOn)";

pub struct OnboardingAction<'a> {
    pub network_id: &'a str,
}

impl OnboardingAction<'_> {
    pub async fn sign(&self) -> () {}

    pub fn getHash(&self, methods: &str, request_path: &str, body: &str, timestamp: &str) -> () {
        //getstructHash
        let hash_actoion = keccak(EIP712_ONBOARDING_ACTION_STRUCT_STRING_TESTNET);
        let hash_methods = keccak(methods);
        let hash_path = keccak(request_path);

        let input_struct = vec![
            SolidityDataType::Bytes(hash_actoion.as_bytes()),
            SolidityDataType::Bytes(hash_methods.as_bytes()),
        ];
        let (_bytes, hash) = abi::encode_packed(&input_struct);

        let struct_hash = keccak(_bytes);
        ()
    }
}
