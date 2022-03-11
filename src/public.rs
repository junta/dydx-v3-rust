use reqwest;
use std::collections::HashMap;

#[tokio::main]
pub async fn get_markets() -> Result<(), Box<dyn std::error::Error>> {
    const DOMAIN: &str = "https://api.dydx.exchange";
    let mut domain = String::new();
    domain.push_str(&DOMAIN);
    domain.push_str(&"/v3/markets");

    let resp = reqwest::get(domain)
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
