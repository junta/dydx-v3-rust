use std::collections::HashMap;

const HTTP_HOST: &str = 'https://api.dydx.exchange'

fn getMarkets() -> Result<(), Box<std::error::Error>> {

    let resp: HashMap<String, String> = reqwest::get(HTTP_HOST)?
        .json()?;
    println!("{:#?}", resp);
    
    Ok(())
}