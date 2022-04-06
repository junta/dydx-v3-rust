use serde::Deserialize;

#[derive(Debug)]
pub struct ApiKeyCredentials<'a> {
    pub key: &'a str,
    pub secret: &'a str,
    pub passphrase: &'a str,
}

#[non_exhaustive]
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OrderSide;

impl OrderSide {
    pub const BUY: &'static str = "BUY";
    pub const SELL: &'static str = "SELL";
}

pub enum Side {
    BUY,
    SEll,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketsResponse {
    pub markets: Markets,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct DydxMarket;

impl DydxMarket {
    pub const BTC_USD: &'static str = "BTC-USD";
    pub const SUSHI_USD: &'static str = "SUSHI-USD";
    pub const AVAX_USD: &'static str = "AVAX-USD";
    pub const INCH_USD: &'static str = "1INCH-USD";
    pub const ETH_USD: &'static str = "ETH-USD";
    pub const XMR_USD: &'static str = "XMR-USD";
    pub const COMP_USD: &'static str = "COMP-USD";
    pub const ALGO_USD: &'static str = "ALGO-USD";
    pub const BCH_USD: &'static str = "BCH-USD";
    pub const CRV_USD: &'static str = "CRV-USD";
    pub const ETC_USD: &'static str = "ETC-USD";
    pub const UNI_USD: &'static str = "UNI-USD";
    pub const MKR_USD: &'static str = "MKR-USD";
    pub const LTC_USD: &'static str = "LTC-USD";
    pub const EOS_USD: &'static str = "EOS-USD";
    pub const DOGE_USD: &'static str = "DOGE-USD";
    pub const ATOM_USD: &'static str = "ATOM-USD";
    pub const ZRX_USD: &'static str = "ZRX-USD";
    pub const SOL_USD: &'static str = "SOL-USD";
    pub const UMA_USD: &'static str = "UMA-USD";
    pub const AAVE_USD: &'static str = "AAVE-USD";
    pub const ADA_USD: &'static str = "ADA-USD";
    pub const SNX_USD: &'static str = "SNX-USD";
    pub const FIL_USD: &'static str = "FIL-USD";
    pub const ZEC_USD: &'static str = "ZEC-USD";
    pub const YFI_USD: &'static str = "YFI-USD";
    pub const XLM_USD: &'static str = "XLM-USD";
    pub const LINK_USD: &'static str = "LINK-USD";
    pub const DOT_USD: &'static str = "DOT-USD";
    pub const MATIC_USD: &'static str = "MATIC-USD";
    pub const ENJ_USD: &'static str = "ENJ-USD";
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    // TODO: change to enum constant
    pub side: String,
    pub size: String,
    pub price: String,
    pub created_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TradesResponse {
    pub trades: Vec<Trade>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    pub started_at: String,
    pub updated_at: String,
    pub market: String,
    // pub resolution: CandleResolution;
    pub resolution: String,
    pub low: String,
    pub high: String,
    pub open: String,
    pub close: String,
    pub base_token_volume: String,
    pub trades: String,
    pub usd_volume: String,
    pub starting_open_interest: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CandlesResponse {
    pub candles: Vec<Candle>,
}
