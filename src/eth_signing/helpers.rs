use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub fn hash_string(input: &str) -> String {
    let mut hasher = Sha3::keccak256();
    hasher.input_str(input);
    let mut hex = hasher.result_str();
    let hex = String::from("0x") + &hex;
    hex
}
