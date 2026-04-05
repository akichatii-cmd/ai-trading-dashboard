use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;

mod components;
mod pages;
mod state;
mod tauri_api;
mod websocket;

use components::*;
use pages::Dashboard;
use websocket::{init_websocket, WsMessage, send_notification};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn playAlert(alert_type: &str);
    
    #[wasm_bindgen(js_namespace = window)]
    fn initAudio();
}
use state::{DashboardState, LogLevel, TradingMode};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let state = create_rw_signal(DashboardState::default());
    provide_context(state);
    
    // Initialize audio on first user interaction
    let init_audio = move |_| {
        initAudio();
    };
    
    // Initialize WebSocket connection
    create_effect(move |_| {
        let ws = init_websocket(move |msg| {
            match msg {
                WsMessage::PriceUpdate { symbol, price, .. } => {
                    state.update(|s| {
                        // Update price for the symbol
                        s.selected_symbol = symbol.clone();
                        // Update position prices if they match
                        for pos in &mut s.positions {
                            if pos.symbol == symbol {
                                pos.current_price = rust_decimal::Decimal::from_f64_retain(price)
                                    .unwrap_or(pos.current_price);
                                // Recalculate PnL
                                let diff = if pos.side == state::OrderSide::Buy {
                                    pos.current_price - pos.entry_price
                                } else {
                                    pos.entry_price - pos.current_price
                                };
                                pos.unrealized_pnl = diff * rust_decimal::Decimal::from(pos.quantity);
                            }
                        }
                    });
                }
                WsMessage::PositionUpdate { id, unrealized_pnl, current_price } => {
                    state.update(|s| {
                        for pos in &mut s.positions {
                            if pos.id == id {
                                pos.unrealized_pnl = rust_decimal::Decimal::from_f64_retain(unrealized_pnl)
                                    .unwrap_or(pos.unrealized_pnl);
                                pos.current_price = rust_decimal::Decimal::from_f64_retain(current_price)
                                    .unwrap_or(pos.current_price);
                            }
                        }
                    });
                }
                WsMessage::SignalGenerated { signal } => {
                    state.update(|s| {
                        s.signals.push(signal.clone());
                        // Keep only last 10 signals
                        if s.signals.len() > 10 {
                            s.signals.remove(0);
                        }
                    });
                    // Play sound and notify
                    playAlert("signal");
                    send_notification(
                        &format!("Signal: {}", signal.symbol),
                        &format!("{:?} @ {}", signal.action, signal.price),
                        None,
                    );
                }
                WsMessage::OrderFilled { order_id, fill_price, filled_qty } => {
                    state.update(|s| {
                        for order in &mut s.orders {
                            if order.id == order_id {
                                order.filled_quantity = filled_qty;
                                if order.filled_quantity >= order.quantity {
                                    order.status = state::OrderStatus::Filled;
                                } else {
                                    order.status = state::OrderStatus::PartiallyFilled;
                                }
                                
                                // Play sound for completed orders
                                if order.filled_quantity >= order.quantity {
                                    playAlert("order_filled");
                                    send_notification(
                                        "Order Filled",
                                        &format!("{} {} @ {}", order.symbol, order.side, fill_price),
                                        None,
                                    );
                                }
                            }
                        }
                    });
                }
                WsMessage::OrderUpdate { order } => {
                    state.update(|s| {
                        // Find and update or add new order
                        let mut found = false;
                        for o in &mut s.orders {
                            if o.id == order.id {
                                *o = order.clone();
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            s.orders.push(order);
                        }
                    });
                }
                WsMessage::RiskAlert { level, message } => {
                    // Log risk alerts
                    state.update(|s| {
                        let log_level = match level.as_str() {
                            "CRITICAL" | "ERROR" => LogLevel::Error,
                            "WARNING" | "WARN" => LogLevel::Warn,
                            _ => LogLevel::Info,
                        };
                        s.logs.push(state::LogEntry {
                            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
                            level: log_level,
                            message: format!("[RISK {}] {}", level, message),
                            source: "risk".to_string(),
                        });
                    });
                    
                    // Play alert sound for critical/warning
                    if level == "CRITICAL" || level == "ERROR" {
                        playAlert("emergency");
                        send_notification("RISK ALERT", &message, None);
                    } else if level == "WARNING" || level == "WARN" {
                        playAlert("risk_alert");
                    }
                }
                WsMessage::SystemStatus { mode, running, health } => {
                    state.update(|s| {
                        s.system.running = running;
                        s.system.health.status = health;
                        // Parse mode
                        s.system.mode = match mode.as_str() {
                            "live" | "LIVE" => TradingMode::Live,
                            "paper" | "PAPER" => TradingMode::Paper,
                            "demo" | "DEMO" => TradingMode::Demo,
                            _ => TradingMode::Off,
                        };
                    });
                }
                WsMessage::LogAppend { level, message, source, .. } => {
                    state.update(|s| {
                        let log_level = match level.as_str() {
                            "ERROR" => LogLevel::Error,
                            "WARN" => LogLevel::Warn,
                            "DEBUG" => LogLevel::Debug,
                            _ => LogLevel::Info,
                        };
                        s.logs.push(state::LogEntry {
                            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
                            level: log_level,
                            message,
                            source,
                        });
                        // Keep only last 1000 logs
                        if s.logs.len() > 1000 {
                            s.logs.remove(0);
                        }
                    });
                }
                _ => {}
            }
        });
        
        // Cleanup on effect re-run (component unmount)
        move || {
            ws.disconnect();
        }
    });
    
    // Also poll API as fallback for initial data
    create_effect(move |_| {
        spawn_local(async move {
            // Initial data load from Tauri API
            if let Ok(status) = tauri_api::get_bot_status().await {
                state.update(|s| {
                    s.system = status;
                });
            }
            
            if let Ok(positions) = tauri_api::get_positions().await {
                state.update(|s| {
                    s.positions = positions;
                });
            }
            
            if let Ok(orders) = tauri_api::get_orders().await {
                state.update(|s| {
                    s.orders = orders;
                });
            }
            
            if let Ok(signals) = tauri_api::get_signals().await {
                state.update(|s| {
                    s.signals = signals;
                });
            }
            
            if let Ok(logs) = tauri_api::get_logs(Some(100)).await {
                state.update(|s| {
                    s.logs = logs;
                });
            }
        });
    });
    
    view! {
        <div on:click=init_audio on:keydown=init_audio>
            <Router>
                <Routes>
                    <Route path="/" view=Dashboard/>
                </Routes>
            </Router>
        </div>
    }
}
