use reqwest;

const HOST: &str = "https://api.dydx.exchange";

#[tokio::main]
async fn get_request(requestPath: &'static str, params: &'static str) {}

pub async fn get_markets(
    market: &'static str,
) -> Result<(serde_json::Value), Box<dyn std::error::Error>> {
    println!("{}", market);
    let path = "v3/markets";
    let mut domain = String::new();
    domain.push_str(&HOST);
    domain.push_str(&"/v3/markets?market=");
    domain.push_str(&market);

    println!("{}", domain);

    let resp: serde_json::Value = reqwest::get(domain).await?.json().await?;
    // dbg!(resp);
    Ok(resp)
}

pub async fn get_orderbook(market: &'static str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub async fn get_trades(market: &'static str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
