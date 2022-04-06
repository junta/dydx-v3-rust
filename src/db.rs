use uuid::Uuid;

pub fn get_user_id() -> Result<(), uuid::Error> {
    // let my_uuid = Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8")?;
    // println!("{}", my_uuid.to_urn());
    let uuid = Uuid::parse_str("0f9da948-a6fb-4c45-9edc-4685c3f3317d").unwrap();
    println!("{}", uuid);
    Ok(())
}
