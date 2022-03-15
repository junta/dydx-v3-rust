pub enum Api {
    Production,
    Staging,
}

impl Api {
    pub fn url(&self) -> &str {
        match &self {
            Api::Production => "https://api.dydx.exchange/v3/",
            Api::Staging => "https://api.stage.dydx.exchange/v3/",
        }
    }
}
