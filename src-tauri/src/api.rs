use crate::models::*;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use std::sync::Arc;
use tokio::sync::RwLock;
use once_cell::sync::Lazy;
use uuid::Uuid;
use serde::Deserialize;

// Global state shared between API and WebSocket
pub static BOT_STATE: Lazy<Arc<RwLock<SystemState>>> = Lazy::new(|| {
    Arc::new(RwLock::new(SystemState {
        version: "5.0.0".to_string(),
        mode: TradingMode::Off,
        running: false,
        demo_mode: true,
        health: HealthStatus {
            status: "initializing".to_string(),
            latency_ms: 0,
        },
        timestamp: chrono::Utc::now(),
    }))
});

static PORTFOLIO: Lazy<Arc<RwLock<Portfolio>>> = Lazy::new(|| {
    Arc::new(RwLock::new(Portfolio {
        total_equity: Decimal::new(500000, 2),
        daily_pnl: Decimal::ZERO,
        daily_pnl_pct: Decimal::ZERO,
        peak_equity: Decimal::new(500000, 2),
        drawdown_pct: Decimal::ZERO,
        available_balance: Decimal::new(500000, 2),
    }))
});

static POSITIONS: Lazy<Arc<RwLock<Vec<Position>>>> = Lazy::new(|| {
    Arc::new(RwLock::new(vec![]))
});

static ORDERS: Lazy<Arc<RwLock<Vec<Order>>>> = Lazy::new(|| {
    Arc::new(RwLock::new(vec![]))
});

static SIGNALS: Lazy<Arc<RwLock<Vec<TradeSignal>>>> = Lazy::new(|| {
    Arc::new(RwLock::new(vec![]))
});

static LOGS: Lazy<Arc<RwLock<Vec<LogEntry>>>> = Lazy::new(|| {
    Arc::new(RwLock::new(vec![]))
});

// Price history cache for chart data
static PRICE_HISTORY: Lazy<Arc<RwLock<std::collections::HashMap<String, Vec<Candle>>>>> = Lazy::new(|| {
    Arc::new(RwLock::new(std::collections::HashMap::new()))
});

// Helper to add log entry
pub async fn add_log(level: LogLevel, message: &str, source: &str) {
    let mut logs = LOGS.write().await;
    logs.push(LogEntry {
        timestamp: chrono::Utc::now(),
        level,
        message: message.to_string(),
        source: source.to_string(),
    });
    // Keep only last 1000 logs
    if logs.len() > 1000 {
        logs.remove(0);
    }
}

#[tauri::command]
pub async fn get_bot_status() -> Result<SystemState, String> {
    let state = BOT_STATE.read().await;
    Ok(state.clone())
}

#[tauri::command]
pub async fn start_trading(args: StartTradingArgs) -> Result<String, String> {
    let mut state = BOT_STATE.write().await;
    state.mode = match args.mode.as_str() {
        "live" => TradingMode::Live,
        "paper" => TradingMode::Paper,
        "demo" => TradingMode::Demo,
        _ => TradingMode::Demo,
    };
    state.running = true;
    
    add_log(LogLevel::Info, &format!("Trading started in {} mode", args.mode), "api").await;
    Ok(format!("Trading started in {} mode", args.mode))
}

#[tauri::command]
pub async fn stop_trading() -> Result<String, String> {
    let mut state = BOT_STATE.write().await;
    state.running = false;
    state.mode = TradingMode::Off;
    
    add_log(LogLevel::Info, "Trading stopped", "api").await;
    Ok("Trading stopped".to_string())
}

#[tauri::command]
pub async fn emergency_stop() -> Result<String, String> {
    let mut state = BOT_STATE.write().await;
    state.running = false;
    state.mode = TradingMode::Off;
    
    // Close all positions
    let mut positions = POSITIONS.write().await;
    positions.clear();
    
    add_log(LogLevel::Error, "EMERGENCY STOP ACTIVATED - All positions closed", "api").await;
    Ok("Emergency stop executed".to_string())
}

#[tauri::command]
pub async fn get_positions() -> Result<Vec<Position>, String> {
    let positions = POSITIONS.read().await;
    Ok(positions.clone())
}

#[tauri::command]
pub async fn get_orders() -> Result<Vec<Order>, String> {
    let orders = ORDERS.read().await;
    Ok(orders.clone())
}

#[tauri::command]
pub async fn close_position(args: ClosePositionArgs) -> Result<String, String> {
    let mut positions = POSITIONS.write().await;
    positions.retain(|p| p.id != args.position_id);
    
    add_log(LogLevel::Info, &format!("Position {} closed", args.position_id), "api").await;
    Ok(format!("Position {} closed", args.position_id))
}

#[tauri::command]
pub async fn place_order(args: PlaceOrderArgs) -> Result<String, String> {
    let order = Order {
        id: format!("order-{}", Uuid::new_v4()),
        symbol: args.symbol.clone(),
        side: match args.side.as_str() {
            "buy" => OrderSide::Buy,
            _ => OrderSide::Sell,
        },
        order_type: match args.order_type.as_str() {
            "market" => OrderType::Market,
            "limit" => OrderType::Limit,
            "stop" => OrderType::Stop,
            _ => OrderType::Market,
        },
        quantity: args.quantity,
        price: args.price.and_then(|p| p.parse().ok()),
        filled_quantity: 0,
        status: OrderStatus::Pending,
        created_at: chrono::Utc::now(),
        time_in_force: "GTC".to_string(),
    };
    
    let mut orders = ORDERS.write().await;
    orders.push(order);
    
    add_log(LogLevel::Info, &format!("Order placed: {} {} {}", args.side, args.symbol, args.quantity), "api").await;
    Ok("Order placed successfully".to_string())
}

#[tauri::command]
pub async fn get_signals() -> Result<Vec<TradeSignal>, String> {
    let signals = SIGNALS.read().await;
    Ok(signals.clone())
}

#[tauri::command]
pub async fn get_logs(args: GetLogsArgs) -> Result<Vec<LogEntry>, String> {
    let logs = LOGS.read().await;
    let limit = args.limit.unwrap_or(100);
    let result: Vec<LogEntry> = logs.iter().rev().take(limit).cloned().collect();
    Ok(result)
}

/// Get historical candlestick data for TradingView chart
#[tauri::command]
pub async fn get_historical_data(
    symbol: String,
    timeframe: String,
    limit: Option<usize>,
    from: Option<i64>,
    to: Option<i64>,
) -> Result<Vec<Candle>, String> {
    let cache_key = format!("{}_{}", symbol, timeframe);
    let history = PRICE_HISTORY.read().await;
    
    // Check if we have cached data
    if let Some(candles) = history.get(&cache_key) {
        let limit = limit.unwrap_or(200);
        let result: Vec<Candle> = candles.iter().rev().take(limit).cloned().rev().collect();
        return Ok(result);
    }
    drop(history);
    
    // Generate mock historical data
    let candles = generate_mock_candles(&symbol, &timeframe, limit.unwrap_or(200));
    
    // Cache the data
    let mut history = PRICE_HISTORY.write().await;
    history.insert(cache_key, candles.clone());
    
    Ok(candles)
}

/// Modify position SL/TP levels
#[tauri::command]
pub async fn modify_position_sl_tp(
    position_id: String,
    stop_loss: Option<f64>,
    take_profit: Option<f64>,
    trailing_stop: Option<f64>,
) -> Result<String, String> {
    let mut positions = POSITIONS.write().await;
    
    if let Some(position) = positions.iter_mut().find(|p| p.id == position_id) {
        if let Some(sl) = stop_loss {
            position.stop_loss = Some(Decimal::from_f64_retain(sl).unwrap_or_default());
        }
        if let Some(tp) = take_profit {
            position.take_profit = Some(Decimal::from_f64_retain(tp).unwrap_or_default());
        }
        if let Some(ts) = trailing_stop {
            position.trailing_stop = Some(Decimal::from_f64_retain(ts).unwrap_or_default());
        }
        
        add_log(
            LogLevel::Info,
            &format!("Modified SL/TP for position {}: SL={:?}, TP={:?}, Trail={:?}", 
                position_id, stop_loss, take_profit, trailing_stop),
            "api"
        ).await;
        
        Ok(format!("Position {} SL/TP updated", position_id))
    } else {
        Err(format!("Position {} not found", position_id))
    }
}

// Generate mock candlestick data
fn generate_mock_candles(symbol: &str, timeframe: &str, count: usize) -> Vec<Candle> {
    let mut candles = Vec::with_capacity(count);
    let base_price = match symbol {
        "SBER" => 250.0,
        "GAZP" => 165.0,
        "LKOH" => 6500.0,
        "YNDX" => 3800.0,
        "TCSG" => 2800.0,
        "MOEX" => 3200.0,
        "PLZL" => 12000.0,
        "MGNT" => 7500.0,
        _ => 100.0,
    };
    
    let interval_secs = timeframe_to_seconds(timeframe);
    let now = chrono::Utc::now().timestamp();
    let start_time = now - (count as i64 * interval_secs);
    
    let mut price = base_price;
    
    for i in 0..count {
        let time = start_time + (i as i64 * interval_secs);
        let change = (rand::random::<f64>() - 0.5) * base_price * 0.02; // ±1% of base
        
        let open = price;
        let close = open + change;
        let high = open.max(close) + rand::random::<f64>() * base_price * 0.005;
        let low = open.min(close) - rand::random::<f64>() * base_price * 0.005;
        let volume = (rand::random::<f64>() * 1000000.0) as u64;
        
        candles.push(Candle::new(time, open, high, low, close, volume));
        price = close;
    }
    
    candles
}

fn timeframe_to_seconds(tf: &str) -> i64 {
    match tf {
        "1m" => 60,
        "5m" => 300,
        "15m" => 900,
        "30m" => 1800,
        "1h" => 3600,
        "4h" => 14400,
        "1d" => 86400,
        _ => 300,
    }
}

// Internal functions for WebSocket updates
pub async fn update_price(symbol: &str, price: Decimal) {
    // Update positions PnL
    let mut positions = POSITIONS.write().await;
    for pos in positions.iter_mut() {
        if pos.symbol == symbol {
            pos.current_price = price;
            let diff = if pos.side == OrderSide::Buy {
                price - pos.entry_price
            } else {
                pos.entry_price - price
            };
            pos.unrealized_pnl = diff * Decimal::from(pos.quantity);
            let entry_total = pos.entry_price * Decimal::from(pos.quantity);
            if entry_total > Decimal::ZERO {
                pos.unrealized_pnl_pct = (pos.unrealized_pnl / entry_total) * Decimal::from(100);
            }
        }
    }
    drop(positions);
    
    // Update price history for chart
    update_price_history(symbol, price).await;
}

async fn update_price_history(symbol: &str, price: Decimal) {
    let timeframes = vec!["1m", "5m", "15m", "1h", "4h", "1d"];
    let mut history = PRICE_HISTORY.write().await;
    
    for tf in timeframes {
        let cache_key = format!("{}_{}", symbol, tf);
        if let Some(candles) = history.get_mut(&cache_key) {
            if let Some(last) = candles.last_mut() {
                let price_f64 = price.to_f64().unwrap_or(0.0);
                last.close = price_f64;
                if price_f64 > last.high {
                    last.high = price_f64;
                }
                if price_f64 < last.low {
                    last.low = price_f64;
                }
            }
        }
    }
}

pub async fn add_signal(signal: TradeSignal) {
    let mut signals = SIGNALS.write().await;
    signals.push(signal);
    if signals.len() > 10 {
        signals.remove(0);
    }
}

// ============================================================================
// Command argument structures for Tauri v2
// ============================================================================

#[derive(Deserialize)]
pub struct StartTradingArgs {
    pub mode: String,
}

#[derive(Deserialize)]
pub struct ClosePositionArgs {
    pub position_id: String,
}

#[derive(Deserialize)]
pub struct PlaceOrderArgs {
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub quantity: u32,
    pub price: Option<String>,
}

#[derive(Deserialize)]
pub struct GetLogsArgs {
    pub limit: Option<usize>,
}
