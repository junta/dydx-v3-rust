use serde::Deserialize;

#[derive(Debug)]
pub struct ApiKeyCredentials<'a> {
    pub key: &'a str,
    pub secret: &'a str,
    pub passphrase: &'a str,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketsResponse {
    pub markets: Markets,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Markets {
    #[serde(rename = "BTC-USD")]
    pub btc_usd: Option<MarketData>,
    #[serde(rename = "SUSHI-USD")]
    pub sushi_usd: Option<MarketData>,
    #[serde(rename = "AVAX-USD")]
    pub avax_usd: Option<MarketData>,
    #[serde(rename = "1INCH-USD")]
    pub inch_usd: Option<MarketData>,
    #[serde(rename = "ETH-USD")]
    pub eth_usd: Option<MarketData>,
    #[serde(rename = "XMR-USD")]
    pub xmr_usd: Option<MarketData>,
    #[serde(rename = "COMP-USD")]
    pub comp_usd: Option<MarketData>,
    #[serde(rename = "ALGO-USD")]
    pub algo_usd: Option<MarketData>,
    #[serde(rename = "BCH-USD")]
    pub bch_usd: Option<MarketData>,
    #[serde(rename = "CRV-USD")]
    pub crv_usd: Option<MarketData>,
    #[serde(rename = "ETC-USD")]
    pub etc_usd: Option<MarketData>,
    #[serde(rename = "UNI-USD")]
    pub uni_usd: Option<MarketData>,
    #[serde(rename = "MKR-USD")]
    pub mkr_usd: Option<MarketData>,
    #[serde(rename = "LTC-USD")]
    pub ltc_usd: Option<MarketData>,
    #[serde(rename = "EOS-USD")]
    pub eos_usd: Option<MarketData>,
    #[serde(rename = "DOGE-USD")]
    pub doge_usd: Option<MarketData>,
    #[serde(rename = "ATOM-USD")]
    pub atom_usd: Option<MarketData>,
    #[serde(rename = "ZRX-USD")]
    pub zrx_usd: Option<MarketData>,
    #[serde(rename = "SOL-USD")]
    pub sol_usd: Option<MarketData>,
    #[serde(rename = "UMA-USD")]
    pub uma_usd: Option<MarketData>,
    #[serde(rename = "AAVE-USD")]
    pub aave_usd: Option<MarketData>,
    #[serde(rename = "ADA-USD")]
    pub ada_usd: Option<MarketData>,
    #[serde(rename = "SNX-USD")]
    pub snx_usd: Option<MarketData>,
    #[serde(rename = "FIL-USD")]
    pub fil_usd: Option<MarketData>,
    #[serde(rename = "ZEC-USD")]
    pub zec_usd: Option<MarketData>,
    #[serde(rename = "YFI-USD")]
    pub yfi_usd: Option<MarketData>,
    #[serde(rename = "XLM-USD")]
    pub xlm_usd: Option<MarketData>,
    #[serde(rename = "LINK-USD")]
    pub link_usd: Option<MarketData>,
    #[serde(rename = "DOT-USD")]
    pub dot_usd: Option<MarketData>,
    #[serde(rename = "MATIC-USD")]
    pub matic_usd: Option<MarketData>,
    #[serde(rename = "ENJ-USD")]
    pub enj_usd: Option<MarketData>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketData {
    pub market: String,
    pub status: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub step_size: String,
    pub tick_size: String,
    pub index_price: String,
    pub oracle_price: String,
    #[serde(rename = "priceChange24H")]
    pub price_change24h: String,
    pub next_funding_rate: String,
    pub next_funding_at: String,
    pub min_order_size: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub initial_margin_fraction: String,
    pub maintenance_margin_fraction: String,
    #[serde(rename = "volume24H")]
    pub volume24h: String,
    #[serde(rename = "trades24H")]
    pub trades24h: String,
    pub open_interest: String,
    pub incremental_initial_margin_fraction: String,
    pub incremental_position_size: String,
    pub max_position_size: String,
    pub baseline_position_size: String,
    pub asset_resolution: String,
    pub synthetic_asset_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookResponse {
    pub asks: Vec<OrderbookResponseOrder>,
    pub bids: Vec<OrderbookResponseOrder>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookResponseOrder {
    pub size: String,
    pub price: String,
}
