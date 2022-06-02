use rand::Rng;
use uuid::Uuid;

const UUID_NAMESPACE: &str = "0f9da948-a6fb-4c45-9edc-4685c3f3317d";

pub fn get_user_id(address: &str) -> String {
    let ethaddress = address.to_lowercase();
    let uuid = Uuid::parse_str(UUID_NAMESPACE).unwrap();
    let user_id = Uuid::new_v5(&uuid, ethaddress.as_bytes());
    user_id.to_string()
}

pub fn get_account_id(address: &str) -> String {
    let user_id = get_user_id(address);
    let user_accout = user_id + "0";
    let uuid = Uuid::parse_str(UUID_NAMESPACE).unwrap();
    let account_id = Uuid::new_v5(&uuid, user_accout.as_bytes());
    account_id.to_string()
}

pub fn generate_random_client_id() -> String {
    let mut rng = rand::thread_rng();
    let rand_number: u32 = rng.gen();
    rand_number.to_string()
}