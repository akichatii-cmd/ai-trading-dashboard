use crate::api::*;
use crate::models::*;
use crate::ws_server::{WsServerState, broadcast_signal, broadcast_log as ws_broadcast_log};
use std::sync::Arc;
use tauri::AppHandle;
use tokio::time::{interval, Duration, sleep};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures::{SinkExt, StreamExt};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use serde_json::json;
use tracing::{info, warn, error};

const BOT_WS_URL: &str = "ws://127.0.0.1:8080/ws";
const RECONNECT_DELAY: Duration = Duration::from_secs(5);

/// Подключение к внешнему торговому боту с трансляцией в наш WS сервер
pub async fn connect_to_bot_with_state(app: AppHandle, ws_state: Arc<WsServerState>) {
    loop {
        info!("Connecting to trading bot at {}...", BOT_WS_URL);
        
        match connect_async(BOT_WS_URL).await {
            Ok((ws_stream, _)) => {
                info!("Connected to trading bot");
                add_log(LogLevel::Info, "Connected to trading bot", "websocket").await;
                
                let (mut write, mut read) = ws_stream.split();
                
                // Send initial subscription message
                let subscribe_msg = json!({
                    "action": "subscribe",
                    "channels": ["prices", "positions", "orders", "signals", "logs"]
                });
                let _ = write.send(Message::Text(subscribe_msg.to_string())).await;
                
                // Handle incoming messages
                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(Message::Text(text)) => {
                            handle_message_with_state(&text, &app, &ws_state).await;
                        }
                        Ok(Message::Close(_)) => {
                            warn!("WebSocket closed by server");
                            break;
                        }
                        Err(e) => {
                            error!("WebSocket error: {}", e);
                            break;
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                warn!("Failed to connect to bot: {}. Retrying in {:?}...", e, RECONNECT_DELAY);
            }
        }
        
        add_log(LogLevel::Warn, "Disconnected from bot, reconnecting...", "websocket").await;
        sleep(RECONNECT_DELAY).await;
    }
}

async fn handle_message_with_state(text: &str, _app: &AppHandle, ws_state: &WsServerState) {
    // Try to parse as generic JSON first
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(text) {
        let msg_type = value.get("type").and_then(|v| v.as_str());
        
        match msg_type {
            Some("price") => {
                if let (Some(symbol), Some(price)) = (
                    value.get("symbol").and_then(|v| v.as_str()),
                    value.get("price").and_then(|v| v.as_str())
                ) {
                    if let Ok(price_dec) = price.parse::<Decimal>() {
                        update_price(symbol, price_dec).await;
                    }
                }
            }
            Some("signal") => {
                if let Ok(signal) = serde_json::from_value::<TradeSignal>(value.get("data").cloned().unwrap_or_default()) {
                    // Транслируем в наш WebSocket сервер
                    broadcast_signal(ws_state.clone(), signal.clone()).await;
                    
                    add_signal(signal).await;
                    add_log(LogLevel::Info, &format!("New signal: {} {}", 
                        match signal.action {
                            SignalAction::Buy => "BUY",
                            SignalAction::Sell => "SELL",
                            SignalAction::Hold => "HOLD",
                        }, 
                        signal.symbol), "websocket").await;
                }
            }
            Some("position") => {
                // Handle position updates
            }
            Some("order") => {
                // Handle order updates
            }
            Some("log") => {
                if let (Some(level), Some(msg)) = (
                    value.get("level").and_then(|v| v.as_str()),
                    value.get("message").and_then(|v| v.as_str())
                ) {
                    let log_level = match level {
                        "ERROR" => LogLevel::Error,
                        "WARN" => LogLevel::Warn,
                        "DEBUG" => LogLevel::Debug,
                        _ => LogLevel::Info,
                    };
                    add_log(log_level, msg, "bot").await;
                }
            }
            _ => {
                // Try parsing as SystemState (backward compatibility)
                if let Ok(state) = serde_json::from_str::<SystemState>(text) {
                    let mut bot_state = BOT_STATE.write().await;
                    *bot_state = state;
                }
            }
        }
    }
}

/// Legacy: Подключение без состояния (для обратной совместимости)
pub async fn connect_to_bot(app: AppHandle) {
    let ws_state = Arc::new(WsServerState::new());
    connect_to_bot_with_state(app, ws_state).await;
}

async fn handle_message(text: &str, _app: &AppHandle) {
    // Legacy обработчик
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(text) {
        let msg_type = value.get("type").and_then(|v| v.as_str());
        
        match msg_type {
            Some("price") => {
                if let (Some(symbol), Some(price)) = (
                    value.get("symbol").and_then(|v| v.as_str()),
                    value.get("price").and_then(|v| v.as_str())
                ) {
                    if let Ok(price_dec) = price.parse::<Decimal>() {
                        update_price(symbol, price_dec).await;
                    }
                }
            }
            Some("signal") => {
                if let Ok(signal) = serde_json::from_value::<TradeSignal>(value.get("data").cloned().unwrap_or_default()) {
                    add_signal(signal).await;
                }
            }
            _ => {}
        }
    }
}

// Spawn mock data generator for demo
pub async fn spawn_mock_data_generator() {
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(1));
        let symbols = vec!["SBER", "GAZP", "LKOH", "YNDX", "TCSG"];
        
        loop {
            ticker.tick().await;
            
            // Generate mock price updates
            for symbol in &symbols {
                let base_price = match *symbol {
                    "SBER" => 250.0,
                    "GAZP" => 165.0,
                    "LKOH" => 6500.0,
                    "YNDX" => 3800.0,
                    "TCSG" => 2800.0,
                    _ => 100.0,
                };
                
                let variation = (rand::random::<f64>() - 0.5) * 0.02; // ±1%
                let price = Decimal::from_f64_retain(base_price * (1.0 + variation)).unwrap_or_default();
                update_price(symbol, price).await;
            }
        }
    });
}
