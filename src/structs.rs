use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy)]
pub struct ApiKeyCredentials<'a> {
    pub key: &'a str,
    pub secret: &'a str,
    pub passphrase: &'a str,
}

// pub struct KeyPair<'a> {
//     pub public_key: &'a str,
//     pub public_key_ycoordinate: &'a str,
//     pub private_key: &'a str,
// }

// #[non_exhaustive]
// #[derive(Default, Debug, Clone, PartialEq, Deserialize)]
// pub struct OrderSide;

// impl OrderSide {
//     pub const BUY: &'static str = "BUY";
//     pub const SELL: &'static str = "SELL";
// }

// pub enum Side {
//     BUY,
//     SEll,
// }

// pub struct Side;

// impl Side {
//     pub const BUY: &'static str = "BUY";
//     pub const SELL: &'static str = "SELL";
// }
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AccountResponse {
    pub account: AccountObject,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AccountsResponse {
    pub accounts: Vec<AccountObject>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountObject {
    pub stark_key: String,
    pub position_id: String,
    pub equity: String,
    pub free_collateral: String,
    pub pending_deposits: String,
    pub pending_withdrawals: String,
    pub open_positions: PositionsMap,
    pub account_number: String,
    pub id: String,
    pub quote_balance: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PositionsResponse {
    pub positions: Vec<PositionResponseObject>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionResponseObject {
    // pub market: Market;
    pub market: String,
    // pub status: PositionStatus,
    pub status: String,
    pub side: String,
    pub size: String,
    pub max_size: String,
    pub entry_price: String,
    pub exit_price: Option<String>,
    pub unrealized_pnl: String,
    pub realized_pnl: Option<String>,
    pub created_at: String,
    pub closed_at: Option<String>,
    pub sum_open: Option<String>,
    pub sum_close: Option<String>,
    pub net_funding: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionsMap {
    #[serde(rename = "BTC-USD")]
    pub btc_usd: Option<PositionResponseObject>,
    #[serde(rename = "SUSHI-USD")]
    pub sushi_usd: Option<PositionResponseObject>,
    #[serde(rename = "AVAX-USD")]
    pub avax_usd: Option<PositionResponseObject>,
    #[serde(rename = "1INCH-USD")]
    pub inch_usd: Option<PositionResponseObject>,
    #[serde(rename = "ETH-USD")]
    pub eth_usd: Option<PositionResponseObject>,
    #[serde(rename = "XMR-USD")]
    pub xmr_usd: Option<PositionResponseObject>,
    #[serde(rename = "COMP-USD")]
    pub comp_usd: Option<PositionResponseObject>,
    #[serde(rename = "ALGO-USD")]
    pub algo_usd: Option<PositionResponseObject>,
    #[serde(rename = "BCH-USD")]
    pub bch_usd: Option<PositionResponseObject>,
    #[serde(rename = "CRV-USD")]
    pub crv_usd: Option<PositionResponseObject>,
    #[serde(rename = "ETC-USD")]
    pub etc_usd: Option<PositionResponseObject>,
    #[serde(rename = "UNI-USD")]
    pub uni_usd: Option<PositionResponseObject>,
    #[serde(rename = "MKR-USD")]
    pub mkr_usd: Option<PositionResponseObject>,
    #[serde(rename = "LTC-USD")]
    pub ltc_usd: Option<PositionResponseObject>,
    #[serde(rename = "EOS-USD")]
    pub eos_usd: Option<PositionResponseObject>,
    #[serde(rename = "DOGE-USD")]
    pub doge_usd: Option<PositionResponseObject>,
    #[serde(rename = "ATOM-USD")]
    pub atom_usd: Option<PositionResponseObject>,
    #[serde(rename = "ZRX-USD")]
    pub zrx_usd: Option<PositionResponseObject>,
    #[serde(rename = "SOL-USD")]
    pub sol_usd: Option<PositionResponseObject>,
    #[serde(rename = "UMA-USD")]
    pub uma_usd: Option<PositionResponseObject>,
    #[serde(rename = "AAVE-USD")]
    pub aave_usd: Option<PositionResponseObject>,
    #[serde(rename = "ADA-USD")]
    pub ada_usd: Option<PositionResponseObject>,
    #[serde(rename = "SNX-USD")]
    pub snx_usd: Option<PositionResponseObject>,
    #[serde(rename = "FIL-USD")]
    pub fil_usd: Option<PositionResponseObject>,
    #[serde(rename = "ZEC-USD")]
    pub zec_usd: Option<PositionResponseObject>,
    #[serde(rename = "YFI-USD")]
    pub yfi_usd: Option<PositionResponseObject>,
    #[serde(rename = "XLM-USD")]
    pub xlm_usd: Option<PositionResponseObject>,
    #[serde(rename = "LINK-USD")]
    pub link_usd: Option<PositionResponseObject>,
    #[serde(rename = "DOT-USD")]
    pub dot_usd: Option<PositionResponseObject>,
    #[serde(rename = "MATIC-USD")]
    pub matic_usd: Option<PositionResponseObject>,
    #[serde(rename = "ENJ-USD")]
    pub enj_usd: Option<PositionResponseObject>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiOrder {
    pub market: String,
    pub side: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub size: String,
    pub price: String,
    pub time_in_force: String,
    pub post_only: bool,
    pub limit_fee: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_percent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub id: String,
    pub client_id: Option<String>,
    pub account_id: Option<String>,
    pub market: String,
    pub side: String,
    pub price: String,
    pub trigger_price: Option<String>,
    pub trailing_percent: Option<String>,
    pub size: String,
    pub remaining_size: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub created_at: String,
    pub unfillable_at: Option<String>,
    pub expires_at: Option<String>,
    pub status: String,
    pub time_in_force: String,
    pub post_only: bool,
    pub cancel_reason: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponseObject {
    pub public_id: String,
    pub ethereum_address: String,
    pub is_registered: bool,
    pub email: Option<String>,
    pub username: Option<String>,
    pub user_data: Value,
    pub maker_fee_rate: Option<String>,
    pub taker_fee_rate: Option<String>,
    pub maker_volume30_d: Option<String>,
    pub taker_volume30_d: Option<String>,
    pub fees30_d: Option<String>,
    pub referred_by_affiliate_link: Option<String>,
    pub is_sharing_username: Option<bool>,
    pub is_sharing_address: Option<bool>,
    pub dydx_token_balance: String,
    pub staked_dydx_token_balance: String,
    pub active_staked_dydx_token_balance: String,
    pub is_email_verified: bool,
    pub country: Option<String>,
    pub hedgies_held: Vec<usize>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserResponse {
    pub user: UserResponseObject,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    // pub user_data: Value,
    pub user_data: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sharing_username: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sharing_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
}
