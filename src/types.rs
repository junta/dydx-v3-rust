use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyCredentialsResponseObject {
    pub key: String,
    pub secret: String,
    pub passphrase: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyCredentialsResponse {
    pub api_key: ApiKeyCredentialsResponseObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeysResponse {
    pub api_keys: Vec<ApiKeyResponseObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyResponseObject {
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyCredentials<'a> {
    pub key: &'a str,
    pub secret: &'a str,
    pub passphrase: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyPairWithYCoordinate<'a> {
    pub public_key: &'a str,
    pub public_key_y_coordinate: &'a str,
    pub private_key: &'a str,
}

#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderSide;

impl OrderSide {
    pub const BUY: &'static str = "BUY";
    pub const SELL: &'static str = "SELL";
}

#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderType;

impl OrderType {
    pub const MARKET: &'static str = "MARKET";
    pub const LIMIT: &'static str = "LIMIT";
    pub const STOP_LIMIT: &'static str = "STOP_LIMIT";
    pub const TRAILING_STOP: &'static str = "TRAILING_STOP";
    pub const TAKE_PROFIT: &'static str = "TAKE_PROFIT";
}

#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeInForce;
impl TimeInForce {
    pub const GTT: &'static str = "GTT";
    pub const FOK: &'static str = "FOK";
    pub const IOC: &'static str = "IOC";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketStatsResponse {
    pub markets: HashMap<String, MarketStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketStats {
    pub market: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub base_volume: String,
    pub quote_volume: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct MarketStatisticDay;
impl MarketStatisticDay {
    pub const ONE: &'static str = "1";
    pub const SEVEN: &'static str = "7";
    pub const THIRTY: &'static str = "30";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalFundingResponse {
    pub historical_funding: Vec<HistoricalFunding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalFunding {
    pub market: String,
    pub rate: String,
    pub price: String,
    pub effective_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigResponse {
    pub collateral_asset_id: String,
    pub collateral_token_address: String,
    pub default_maker_fee: String,
    pub default_taker_fee: String,
    pub exchange_address: String,
    pub max_expected_batch_length_minutes: String,
    pub max_fast_withdrawal_amount: String,
    pub cancel_order_rate_limiting: CancelOrderRateLimiting,
    pub place_order_rate_limiting: PlaceOrderRateLimiting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRateLimiting {
    pub max_points_multi: u32,
    pub max_points_single: u32,
    pub window_sec_multi: u32,
    pub window_sec_single: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRateLimiting {
    pub max_points: u32,
    pub window_sec: u32,
    pub target_notional: u32,
    pub min_limit_consumption: u32,
    pub min_market_consumption: u32,
    pub min_triggerable_consumption: u32,
    pub max_order_consumption: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardPnlResponse {
    pub top_pnls: Vec<PNLForPeriod>,
    pub num_participants: u32,
    pub started_at: Option<String>,
    pub ends_at: Option<String>,
    pub updated_at: String,
    pub season_number: Option<u16>,
    pub prize_pool: Option<u32>,
    pub num_hedgies_winners: Option<u16>,
    pub num_prize_winners: Option<u16>,
    pub ratio_promoted: Option<f32>,
    pub ratio_demoted: Option<f32>,
    pub minimum_equity: Option<u16>,
    pub minimum_dydx_tokens: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PNLForPeriod {
    pub username: Option<String>,
    pub ethereum_address: Option<String>,
    pub public_id: String,
    pub absolute_pnl: String,
    pub percent_pnl: String,
    pub absolute_rank: u32,
    pub percent_rank: u32,
    pub season_expected_outcome: Option<String>,
    pub hedgie_won: Option<u16>,
    pub prize_won: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserExistsResponse {
    pub exists: bool,
    pub is_proxy_signer: bool,
    pub contract_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsernameExistsResponse {
    pub exists: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTimeResponse {
    pub iso: String,
    pub epoch: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentlyRevealedHedgies {
    pub daily: HedgiePeriodResponseObject,
    pub weekly: HedgiePeriodResponseObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HedgiePeriodResponse {
    pub historical_token_ids: Vec<HedgiePeriodResponseObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HedgiePeriodResponseObject {
    pub block_number: String,
    pub token_ids: Vec<String>,
    pub competition_period: u16,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NftRevealType;
impl NftRevealType {
    pub const DAY: &'static str = "daily";
    pub const WEEK: &'static str = "weekly";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceFundBalanceResponse {
    pub balance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfilePublicResponse {
    pub username: Option<String>,
    pub ethereum_address: String,
    #[serde(rename = "DYDXHoldings")]
    pub dydx_holdings: String,
    #[serde(rename = "stakedDYDXHoldings")]
    pub staked_dydx_holdings: String,
    pub hedgies_held: Vec<u16>,
    pub twitter_handle: String,
    pub trading_leagues: TradingLeagues,
    pub trading_pnls: TradingPnls,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfilePrivateResponse {
    pub username: Option<String>,
    pub public_id: String,
    pub ethereum_address: String,
    #[serde(rename = "DYDXHoldings")]
    pub dydx_holdings: String,
    #[serde(rename = "stakedDYDXHoldings")]
    pub staked_dydx_holdings: String,
    pub hedgies_held: Vec<u16>,
    pub twitter_handle: String,
    pub affiliate_link: Option<String>,
    pub trading_leagues: TradingLeagues,
    pub trading_pnls: TradingPnls,
    pub trading_rewards: TradingRewards,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingLeagues {
    pub current_league: Option<String>,
    pub current_league_ranking: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingPnls {
    pub absolute_pnl30_d: Option<String>,
    pub percent_pnl30_d: Option<String>,
    pub volume30_d: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingRewards {
    pub cur_epoch: u16,
    pub cur_epoch_estimated_rewards: String,
    pub prev_epoch_estimated_rewards: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketsResponse {
    pub markets: HashMap<String, MarketData>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CandleResolution;
impl CandleResolution {
    pub const ONE_DAY: &'static str = "1DAY";
    pub const FOUR_HOURS: &'static str = "4HOURS";
    pub const ONE_HOUR: &'static str = "1HOUR";
    pub const THIRTY_MINS: &'static str = "30MINS";
    pub const FIFTEEN_MINS: &'static str = "15MINS";
    pub const FIVE_MINS: &'static str = "5MINS";
    pub const ONE_MIN: &'static str = "1MIN";
}

#[derive(Default, Debug, Serialize, Deserialize)]
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
    pub const NEAR_USD: &'static str = "NEAR-USD";
    pub const LUNA_USD: &'static str = "LUNA-USD";
    pub const CELO_USD: &'static str = "CELO-USD";
    pub const XTZ_USD: &'static str = "XTZ-USD";
    pub const RUNE_USD: &'static str = "RUNE-USD";
    pub const TRX_USD: &'static str = "TRX-USD";
    pub const ICP_USD: &'static str = "ICP-USD";
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookResponse {
    pub asks: Vec<OrderbookResponseOrder>,
    pub bids: Vec<OrderbookResponseOrder>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookResponseOrder {
    pub size: String,
    pub price: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub side: String,
    pub size: String,
    pub price: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradesResponse {
    pub trades: Vec<Trade>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandlesResponse {
    pub candles: Vec<Candle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountResponse {
    pub account: AccountObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountsResponse {
    pub accounts: Vec<AccountObject>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountObject {
    pub stark_key: String,
    pub position_id: String,
    pub equity: String,
    pub free_collateral: String,
    pub pending_deposits: String,
    pub pending_withdrawals: String,
    pub open_positions: HashMap<String, PositionResponseObject>,
    pub account_number: String,
    pub id: String,
    pub quote_balance: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoveryResponse {
    pub stark_key: String,
    pub position_id: String,
    pub quote_balance: String,
    pub positions: Vec<PositionResponseObject>,
    pub equity: String,
    pub free_collateral: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionsResponse {
    pub positions: Vec<PositionResponseObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiOrder<'a> {
    pub market: &'a str,
    pub side: &'a str,
    #[serde(rename = "type")]
    pub type_field: &'a str,
    pub time_in_force: &'a str,
    pub post_only: bool,
    pub size: &'a str,
    pub price: &'a str,
    pub limit_fee: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_percent: Option<&'a str>,
    pub expiration: &'a str,
    pub client_id: &'a str,
    pub signature: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApiOrderParams<'a> {
    pub position_id: &'a str,
    pub market: &'a str,
    pub side: &'a str,
    #[serde(rename = "type")]
    pub type_field: &'a str,
    pub size: &'a str,
    pub price: &'a str,
    pub time_in_force: &'a str,
    pub post_only: bool,
    pub limit_fee: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_percent: Option<&'a str>,
    pub expiration: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub order: OrderResponseObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrdersResponse {
    pub orders: Vec<OrderResponseObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveOrdersResponse {
    pub orders: Vec<ActiveOrderResponseObject>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveOrderResponseObject {
    pub id: String,
    pub account_id: String,
    pub market: String,
    pub side: String,
    pub price: String,
    pub remaining_size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillsResponse {
    pub fills: Vec<FillResponseObject>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingResponse {
    pub funding_payments: Vec<FundingResponseObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalPnlResponse {
    pub historical_pnl: Vec<HistoricalPnlResponseObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalPnlResponseObject {
    pub equity: String,
    pub total_pnl: String,
    pub created_at: String,
    pub net_transfers: String,
    pub account_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingResponseObject {
    pub market: String,
    pub payment: String,
    pub rate: String,
    pub position_size: String,
    pub price: String,
    pub effective_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingRewardsResponse {
    pub epoch: u16,
    pub epoch_start: String,
    pub epoch_end: String,
    pub fees: Fees,
    pub open_interest: OpenInterest,
    pub weight: Weight,
    #[serde(rename = "stakedDYDX")]
    pub staked_dydx: StakedDYDX,
    pub total_rewards: String,
    pub estimated_rewards: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weight {
    pub weight: String,
    pub total_weight: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fees {
    pub fees_paid: String,
    pub total_fees_paid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    pub average_open_interest: String,
    pub total_average_open_interest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiquidityProviderRewardsResponse {
    pub epoch: u16,
    pub epoch_start: String,
    pub epoch_end: String,
    pub markets: HashMap<String, LiquidityRewards>,
    #[serde(rename = "stakedDYDX")]
    pub staked_dydx: LiqStakedDYDX,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiquidityRewards {
    pub market: String,
    pub depth_spread_score: String,
    pub uptime: String,
    pub max_uptime: String,
    pub score: String,
    pub total_score: String,
    pub maker_volume: String,
    pub total_maker_volume: String,
    pub total_rewards: String,
    pub estimated_rewards: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiqStakedDYDX {
    #[serde(rename = "averageStakedDYDX")]
    pub average_staked_dydx: String,
    #[serde(rename = "totalAverageStakedDYDX")]
    pub total_average_staked_dydx: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StakedDYDX {
    #[serde(rename = "averageStakedDYDX")]
    pub average_staked_dydx: String,
    #[serde(rename = "averageStakedDYDXWithFloor")]
    pub average_staked_dydxwith_floor: String,
    #[serde(rename = "totalAverageStakedDYDX")]
    pub total_average_staked_dydx: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetroactiveMiningRewardsResponse {
    pub epoch: u16,
    pub epoch_start: String,
    pub epoch_end: String,
    pub retroactive_mining: RetroactiveMiningRewardsResponseObject,
    pub estimated_rewards: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetroactiveMiningRewardsResponseObject {
    pub allocation: String,
    pub target_volume: String,
    pub volume: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicRetroactiveMiningRewardsResponse {
    pub allocation: String,
    pub target_volume: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillResponseObject {
    pub id: String,
    pub side: String,
    pub liquidity: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub market: String,
    pub order_id: String,
    pub price: String,
    pub size: String,
    pub fee: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationResponse {
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountPnlsResponse {
    pub absolute_pnl: String,
    pub percent_pnl: String,
    pub absolute_rank: Option<u16>,
    pub percent_rank: Option<u16>,
    pub started_at: Option<String>,
    pub ends_at: Option<String>,
    pub updated_at: String,
    pub period: String,
    pub season_expected_outcome: Option<String>,
    pub season_number: Option<String>,
    pub hedgie_won: Option<String>,
    pub prize_won: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalLeaderboardPnlsResponse {
    pub leaderboard_pnls: Vec<AccountPnlsResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalLeaderboardPnlsResponseObject {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponseObject {
    pub id: String,
    pub client_id: String,
    pub account_id: String,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub cancel_order: OrderResponseObject,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrdersResponse {
    pub cancel_orders: Vec<OrderResponseObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub user: UserResponseObject,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sharing_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sharing_username: Option<bool>,
    pub user_data: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserParams<'a> {
    pub stark_key: &'a str,
    pub stark_key_y_coordinate: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referred_by_affiliate_link: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferResponseObject {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub debit_asset: String,
    pub credit_asset: String,
    pub debit_amount: String,
    pub credit_amount: String,
    pub transaction_hash: Option<String>,
    pub status: String,
    pub created_at: String,
    pub confirmed_at: Option<String>,
    pub client_id: String,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferResponse {
    pub transfer: TransferResponseObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransfersResponse {
    pub transfers: Vec<TransferResponseObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalResponse {
    pub withdrawal: TransferResponseObject,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccountParams<'a> {
    pub stark_key: &'a str,
    pub stark_key_y_coordinate: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferParams<'a> {
    pub amount: &'a str,
    pub position_id: &'a str,
    pub receiver_account_id: &'a str,
    pub receiver_public_key: &'a str,
    pub receiver_position_id: &'a str,
    pub expiration: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiTransfer<'a> {
    pub amount: &'a str,
    pub receiver_account_id: &'a str,
    pub expiration: &'a str,
    pub client_id: &'a str,
    pub signature: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiWithdrawParams<'a> {
    pub position_id: &'a str,
    pub amount: &'a str,
    pub asset: &'a str,
    pub expiration: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiWithdraw<'a> {
    pub amount: &'a str,
    pub asset: &'a str,
    pub expiration: &'a str,
    pub client_id: &'a str,
    pub signature: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiFastWithdrawalParams<'a> {
    pub position_id: &'a str,
    pub credit_asset: &'a str,
    pub credit_amount: &'a str,
    pub debit_amount: &'a str,
    pub to_address: &'a str,
    pub lp_position_id: &'a str,
    pub lp_stark_key: &'a str,
    pub expiration: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiFastWithdrawal<'a> {
    pub credit_asset: &'a str,
    pub credit_amount: &'a str,
    pub debit_amount: &'a str,
    pub to_address: &'a str,
    pub lp_position_id: &'a str,
    pub expiration: &'a str,
    pub client_id: &'a str,
    pub signature: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserResponse {
    pub api_key: ApiKeyCredentialsResponseObject,
    pub user: UserResponseObject,
    pub account: AccountObject,
}
