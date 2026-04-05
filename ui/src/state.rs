use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DashboardState {
    pub system: SystemState,
    pub portfolio: Portfolio,
    pub positions: Vec<Position>,
    pub orders: Vec<Order>,
    pub signals: Vec<TradeSignal>,
    pub logs: Vec<LogEntry>,
    pub selected_symbol: String,
    pub selected_timeframe: Timeframe,
    pub terminal_collapsed: bool,
    pub active_tab: TerminalTab,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    pub version: String,
    pub mode: TradingMode,
    pub running: bool,
    pub demo_mode: bool,
    pub health: HealthStatus,
}

impl Default for SystemState {
    fn default() -> Self {
        Self {
            version: "5.0.0".to_string(),
            mode: TradingMode::Demo,
            running: false,
            demo_mode: true,
            health: HealthStatus {
                status: "ok".to_string(),
                latency_ms: 45,
            },
        }
    }
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    pub open_time: String,
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
    pub created_at: String,
    pub time_in_force: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub generated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SignalAction {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Timeframe {
    M1, M5, M15, M30, H1, H4, D1, W1
}

impl Default for Timeframe {
    fn default() -> Self {
        Timeframe::M5
    }
}

impl Timeframe {
    pub fn as_str(&self) -> &'static str {
        match self {
            Timeframe::M1 => "1m",
            Timeframe::M5 => "5m",
            Timeframe::M15 => "15m",
            Timeframe::M30 => "30m",
            Timeframe::H1 => "1h",
            Timeframe::H4 => "4h",
            Timeframe::D1 => "1d",
            Timeframe::W1 => "1w",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TerminalTab {
    Logs,
    Terminal,
    Errors,
    Metrics,
    Orders,
    Performance,
}

impl Default for TerminalTab {
    fn default() -> Self {
        TerminalTab::Logs
    }
}
