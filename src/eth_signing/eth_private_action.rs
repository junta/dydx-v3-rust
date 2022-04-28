#![allow(unused)]
use super::helpers::*;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use eth_encode_packed::abi;
use eth_encode_packed::ethabi::ethereum_types::{Address, U256};
use eth_encode_packed::hex;
use eth_encode_packed::SolidityDataType;
use keccak_hash::keccak;

const EIP712_ETH_PRIVATE_ACTION_STRUCT_STRING_STRING: &str =
    "dYdX(string method,string requestPath,string body,string timestamp)";

const EIP712_DOMAIN_STRING_NO_CONTRACT: &str =
    "EIP712Domain(string name,string version,uint256 chainId)";

const EIP712_STRUCT_NAME: &str = "dYdX";
const domain: &str = "dYdX";
const version: &str = "1.0";
pub struct SignEthPrivateAction<'a> {
    pub network_id: &'a str,
}

impl SignEthPrivateAction<'_> {
    // pub fn sign(signer: String, message: String) -> String {}

    pub fn getHash(
        &self,
        methods: &str,
        request_path: &str,
        body: &str,
        timestamp: &str,
    ) -> String {
        //getDomainHash
        let hash_contract = keccak(EIP712_DOMAIN_STRING_NO_CONTRACT);
        let hash_domain = keccak(domain);
        let hash_version = keccak(version);
        let input = vec![
            SolidityDataType::Bytes(hash_contract.as_bytes()),
            SolidityDataType::Bytes(hash_domain.as_bytes()),
            SolidityDataType::Bytes(hash_version.as_bytes()),
            SolidityDataType::Number(U256::from(3_i32)),
        ];
        let (_bytes, hash) = abi::encode_packed(&input);

        let domain_hash = keccak(_bytes);
        // dbg!(domain_hash);

        //getstructHash
        let hash_actoion = keccak(EIP712_ETH_PRIVATE_ACTION_STRUCT_STRING_STRING);
        let hash_methods = keccak(methods);
        let hash_path = keccak(request_path);
        let hash_body = keccak(body);
        let hash_timestamp = keccak(timestamp);

        let input_struct = vec![
            SolidityDataType::Bytes(hash_actoion.as_bytes()),
            SolidityDataType::Bytes(hash_methods.as_bytes()),
            SolidityDataType::Bytes(hash_path.as_bytes()),
            SolidityDataType::Bytes(hash_body.as_bytes()),
            SolidityDataType::Bytes(hash_timestamp.as_bytes()),
        ];
        let (_bytes, hash) = abi::encode_packed(&input_struct);

        let struct_hash = keccak(_bytes);

        // dbg!(struct_hash);

        let prefix: &[u8; 2] = b"\x19\x01";
        let input_eip712 = vec![
            SolidityDataType::Bytes(prefix),
            SolidityDataType::Bytes(domain_hash.as_bytes()),
            SolidityDataType::Bytes(struct_hash.as_bytes()),
        ];
        let (_bytes, hash) = abi::encode_packed(&input_eip712);

        let eip712_hash = keccak(_bytes);
        dbg!(eip712_hash);

        eip712_hash.to_string()
    }
}
