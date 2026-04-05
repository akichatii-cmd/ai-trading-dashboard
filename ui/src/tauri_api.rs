use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};
use crate::state::*;

// Tauri invoke wrapper
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub async fn get_bot_status() -> Result<SystemState, String> {
    let result = invoke("get_bot_status", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to parse status: {}", e))
}

pub async fn start_trading(mode: &str) -> Result<String, String> {
    let args = serde_wasm_bindgen::to_value(&StartTradingArgs { mode: mode.to_string() })
        .map_err(|e| e.to_string())?;
    let result = invoke("start_trading", args).await;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to start trading: {}", e))
}

pub async fn stop_trading() -> Result<String, String> {
    let result = invoke("stop_trading", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to stop trading: {}", e))
}

pub async fn emergency_stop() -> Result<String, String> {
    let result = invoke("emergency_stop", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to emergency stop: {}", e))
}

pub async fn get_positions() -> Result<Vec<Position>, String> {
    let result = invoke("get_positions", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to get positions: {}", e))
}

pub async fn get_orders() -> Result<Vec<Order>, String> {
    let result = invoke("get_orders", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to get orders: {}", e))
}

pub async fn close_position(position_id: &str) -> Result<String, String> {
    let args = serde_wasm_bindgen::to_value(&ClosePositionArgs { 
        position_id: position_id.to_string() 
    }).map_err(|e| e.to_string())?;
    let result = invoke("close_position", args).await;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to close position: {}", e))
}

pub async fn place_order(
    symbol: &str,
    side: &str,
    order_type: &str,
    quantity: u32,
    price: Option<&str>,
) -> Result<String, String> {
    let args = serde_wasm_bindgen::to_value(&PlaceOrderArgs {
        symbol: symbol.to_string(),
        side: side.to_string(),
        order_type: order_type.to_string(),
        quantity,
        price: price.map(|s| s.to_string()),
    }).map_err(|e| e.to_string())?;
    let result = invoke("place_order", args).await;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to place order: {}", e))
}

pub async fn get_signals() -> Result<Vec<TradeSignal>, String> {
    let result = invoke("get_signals", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to get signals: {}", e))
}

pub async fn get_logs(limit: Option<usize>) -> Result<Vec<LogEntry>, String> {
    let args = if let Some(l) = limit {
        serde_wasm_bindgen::to_value(&GetLogsArgs { limit: Some(l) })
            .map_err(|e| e.to_string())?
    } else {
        JsValue::NULL
    };
    let result = invoke("get_logs", args).await;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to get logs: {}", e))
}

#[derive(Serialize)]
struct StartTradingArgs {
    mode: String,
}

#[derive(Serialize)]
struct ClosePositionArgs {
    position_id: String,
}

#[derive(Serialize)]
struct PlaceOrderArgs {
    symbol: String,
    side: String,
    order_type: String,
    quantity: u32,
    price: Option<String>,
}

#[derive(Serialize)]
struct GetLogsArgs {
    limit: Option<usize>,
}

// Initialize WebSocket connection for real-time updates
pub fn init_websocket(state: RwSignal<DashboardState>) {
    spawn_local(async move {
        // TODO: Implement WebSocket connection
        // For now, we'll poll the API every few seconds
        loop {
            gloo_timers::future::TimeoutFuture::new(5000).await;
            
            // Fetch updates
            if let Ok(positions) = get_positions().await {
                state.update(|s| s.positions = positions);
            }
            
            if let Ok(orders) = get_orders().await {
                state.update(|s| s.orders = orders);
            }
            
            if let Ok(signals) = get_signals().await {
                state.update(|s| s.signals = signals);
            }
        }
    });
}

// gloo_timers fallback for setInterval
use gloo_timers::future::TimeoutFuture;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Candle {
    pub time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

pub async fn get_historical_data(symbol: &str, timeframe: &str, limit: Option<usize>) -> Result<Vec<Candle>, String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "symbol": symbol,
        "timeframe": timeframe,
        "limit": limit.unwrap_or(200),
        "from": None::<i64>,
        "to": None::<i64>,
    })).map_err(|e| e.to_string())?;
    
    let result = invoke("get_historical_data", args).await;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to get historical data: {}", e))
}

pub async fn modify_position_sl_tp(
    position_id: &str,
    stop_loss: Option<f64>,
    take_profit: Option<f64>,
    trailing_stop: Option<f64>,
) -> Result<String, String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "position_id": position_id,
        "stop_loss": stop_loss,
        "take_profit": take_profit,
        "trailing_stop": trailing_stop,
    })).map_err(|e| e.to_string())?;
    
    let result = invoke("modify_position_sl_tp", args).await;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to modify SL/TP: {}", e))
}
