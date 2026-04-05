use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    pub version: String,
    pub mode: TradingMode,
    pub running: bool,
    pub demo_mode: bool,
    pub health: HealthStatus,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradingMode {
    Live,
    Paper,
    Demo,
    Off,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub latency_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub total_equity: Decimal,
    pub daily_pnl: Decimal,
    pub daily_pnl_pct: Decimal,
    pub peak_equity: Decimal,
    pub drawdown_pct: Decimal,
    pub available_balance: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub id: String,
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: u32,
    pub entry_price: Decimal,
    pub current_price: Decimal,
    pub unrealized_pnl: Decimal,
    pub unrealized_pnl_pct: Decimal,
    pub stop_loss: Option<Decimal>,
    pub take_profit: Option<Decimal>,
    pub trailing_stop: Option<Decimal>,
    pub open_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: u32,
    pub price: Option<Decimal>,
    pub filled_quantity: u32,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub time_in_force: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Working,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeSignal {
    pub symbol: String,
    pub action: SignalAction,
    pub confidence: f64,
    pub price: Decimal,
    pub stop_loss: Decimal,
    pub take_profit: Decimal,
    pub strategy: String,
    pub timeframe: String,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalAction {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskStatus {
    pub daily_loss_used_pct: Decimal,
    pub positions_used_pct: Decimal,
    pub drawdown_pct: Decimal,
    pub exposure_used_pct: Decimal,
    pub status: RiskSystemStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskSystemStatus {
    AllSystemsGo,
    Caution,
    LimitApproaching,
    TradingLocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketCondition {
    pub volatility: Decimal,
    pub volatility_status: String,
    pub trend_strength: Decimal,
    pub trend_direction: String,
    pub volume_status: String,
    pub correlations: Vec<CorrelationAlert>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationAlert {
    pub pair: String,
    pub correlation: Decimal,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorValues {
    pub rsi_14: Option<Decimal>,
    pub macd_line: Option<Decimal>,
    pub macd_signal: Option<Decimal>,
    pub sma_20: Option<Decimal>,
    pub sma_50: Option<Decimal>,
    pub atr_14: Option<Decimal>,
    pub bb_upper: Option<Decimal>,
    pub bb_lower: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub message: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceUpdate {
    pub symbol: String,
    pub price: Decimal,
    pub bid: Option<Decimal>,
    pub ask: Option<Decimal>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleData {
    pub timestamp: DateTime<Utc>,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}

/// Candlestick for TradingView chart (f64 format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    pub time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

impl Candle {
    pub fn new(time: i64, open: f64, high: f64, low: f64, close: f64, volume: u64) -> Self {
        Self { time, open, high, low, close, volume }
    }
}

/// Historical data request parameters
#[derive(Debug, Clone, Deserialize)]
pub struct HistoricalDataRequest {
    pub symbol: String,
    pub timeframe: String,
    pub limit: Option<usize>,
    pub from: Option<i64>,
    pub to: Option<i64>,
}
