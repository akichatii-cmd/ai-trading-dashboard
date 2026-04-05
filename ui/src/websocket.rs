use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{WebSocket, MessageEvent, ErrorEvent, CloseEvent};
use gloo_timers::future::TimeoutFuture;

const WS_URL: &str = "ws://localhost:8081";
const RECONNECT_BASE_DELAY: u32 = 1000;
const MAX_RECONNECT_DELAY: u32 = 30000;
const HEARTBEAT_INTERVAL: u32 = 30000;

/// Типы WebSocket сообщений
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WsMessage {
    #[serde(rename = "price.update")]
    PriceUpdate { symbol: String, price: f64, ts: u64 },
    
    #[serde(rename = "position.update")]
    PositionUpdate { id: String, unrealized_pnl: f64, current_price: f64 },
    
    #[serde(rename = "signal.new")]
    SignalGenerated { signal: crate::state::TradeSignal },
    
    #[serde(rename = "signal.stale")]
    SignalStale { symbol: String, age_seconds: u64 },
    
    #[serde(rename = "order.fill")]
    OrderFilled { order_id: String, fill_price: f64, filled_qty: u32 },
    
    #[serde(rename = "order.update")]
    OrderUpdate { order: crate::state::Order },
    
    #[serde(rename = "risk.alert")]
    RiskAlert { level: String, message: String },
    
    #[serde(rename = "system.status")]
    SystemStatus { mode: String, running: bool, health: String },
    
    #[serde(rename = "log.append")]
    LogAppend { level: String, message: String, source: String, ts: u64 },
    
    #[serde(rename = "ping")]
    Ping,
    
    #[serde(rename = "pong")]
    Pong,
}

/// Callback для получения сообщений
type MessageCallback = Box<dyn Fn(WsMessage)>;
type StatusCallback = Box<dyn Fn(ConnectionStatus)>;

/// Статус подключения
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionStatus {
    Connecting,
    Connected,
    Disconnected,
    Reconnecting { attempt: u32, delay_ms: u32 },
    Error,
}

/// WebSocket клиент
pub struct WsClient {
    ws: Rc<RefCell<Option<WebSocket>>>,
    message_callback: Rc<RefCell<Option<MessageCallback>>>,
    status_callback: Rc<RefCell<Option<StatusCallback>>>,
    reconnect_attempt: Rc<RefCell<u32>>,
}

impl WsClient {
    pub fn new() -> Self {
        Self {
            ws: Rc::new(RefCell::new(None)),
            message_callback: Rc::new(RefCell::new(None)),
            status_callback: Rc::new(RefCell::new(None)),
            reconnect_attempt: Rc::new(RefCell::new(0)),
        }
    }
    
    pub fn on_message<F>(&self, callback: F)
    where
        F: Fn(WsMessage) + 'static,
    {
        *self.message_callback.borrow_mut() = Some(Box::new(callback));
    }
    
    pub fn on_status_change<F>(&self, callback: F)
    where
        F: Fn(ConnectionStatus) + 'static,
    {
        *self.status_callback.borrow_mut() = Some(Box::new(callback));
    }
    
    pub fn connect(&self) {
        self.set_status(ConnectionStatus::Connecting);
        
        match WebSocket::new(WS_URL) {
            Ok(ws) => {
                ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
                
                let ws_rc = self.ws.clone();
                let msg_cb = self.message_callback.clone();
                let status_cb = self.status_callback.clone();
                let reconnect_rc = self.reconnect_attempt.clone();
                let self_ref = self.clone_instance();
                
                // onopen
                let onopen = Closure::wrap(Box::new(move || {
                    web_sys::console::log_1(&"WebSocket connected".into());
                    *reconnect_rc.borrow_mut() = 0;
                    self_ref.set_status(ConnectionStatus::Connected);
                    
                    // Subscribe to channels
                    let subscribe_msg = serde_json::json!({
                        "type": "subscribe",
                        "data": { "channels": ["prices", "positions", "orders", "signals", "logs", "risks"] }
                    });
                    let _ = ws.send_with_str(&subscribe_msg.to_string());
                }) as Box<dyn FnMut()>);
                ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
                onopen.forget();
                
                // onmessage
                let msg_ws = ws_rc.clone();
                let onmessage = Closure::wrap(Box::new(move |e: MessageEvent| {
                    if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
                        let text_str = text.as_string().unwrap_or_default();
                        
                        match serde_json::from_str::<WsMessage>(&text_str) {
                            Ok(WsMessage::Ping) => {
                                if let Some(ws) = msg_ws.borrow().as_ref() {
                                    let pong = serde_json::json!({ "type": "pong" });
                                    let _ = ws.send_with_str(&pong.to_string());
                                }
                            }
                            Ok(msg) => {
                                if let Some(cb) = msg_cb.borrow().as_ref() {
                                    cb(msg);
                                }
                            }
                            Err(e) => {
                                web_sys::console::error_1(&format!("Parse error: {}", e).into());
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>);
                ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
                onmessage.forget();
                
                // onerror
                let status_cb_err = status_cb.clone();
                let onerror = Closure::wrap(Box::new(move |_e: ErrorEvent| {
                    if let Some(cb) = status_cb_err.borrow().as_ref() {
                        cb(ConnectionStatus::Error);
                    }
                }) as Box<dyn FnMut(_)>);
                ws.set_onerror(Some(onerror.as_ref().unchecked_ref()));
                onerror.forget();
                
                // onclose
                let self_close = self.clone_instance();
                let onclose = Closure::wrap(Box::new(move |_e: CloseEvent| {
                    self_close.schedule_reconnect();
                }) as Box<dyn FnMut(_)>);
                ws.set_onclose(Some(onclose.as_ref().unchecked_ref()));
                onclose.forget();
                
                *self.ws.borrow_mut() = Some(ws);
            }
            Err(e) => {
                web_sys::console::error_1(&format!("Failed to create WebSocket: {:?}", e).into());
                self.set_status(ConnectionStatus::Error);
                self.schedule_reconnect();
            }
        }
    }
    
    pub fn disconnect(&self) {
        if let Some(ws) = self.ws.borrow_mut().take() {
            let _ = ws.close();
        }
        self.set_status(ConnectionStatus::Disconnected);
    }
    
    pub fn send(&self, msg: &str) -> Result<(), JsValue> {
        if let Some(ws) = self.ws.borrow().as_ref() {
            if ws.ready_state() == WebSocket::OPEN {
                return ws.send_with_str(msg);
            }
        }
        Err(JsValue::from_str("WebSocket not connected"))
    }
    
    fn schedule_reconnect(&self) {
        let attempt = *self.reconnect_attempt.borrow();
        *self.reconnect_attempt.borrow_mut() = attempt + 1;
        
        let delay = (RECONNECT_BASE_DELAY * 2_u32.pow(attempt.min(5))).min(MAX_RECONNECT_DELAY);
        
        self.set_status(ConnectionStatus::Reconnecting { 
            attempt: attempt + 1, 
            delay_ms: delay 
        });
        
        let self_reconnect = self.clone_instance();
        spawn_local(async move {
            TimeoutFuture::new(delay).await;
            self_reconnect.connect();
        });
    }
    
    fn set_status(&self, status: ConnectionStatus) {
        if let Some(cb) = self.status_callback.borrow().as_ref() {
            cb(status);
        }
    }
    
    fn clone_instance(&self) -> Self {
        Self {
            ws: self.ws.clone(),
            message_callback: self.message_callback.clone(),
            status_callback: self.status_callback.clone(),
            reconnect_attempt: self.reconnect_attempt.clone(),
        }
    }
}

impl Default for WsClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Request desktop notification permission
pub fn request_notification_permission() {
    let window = web_sys::window().unwrap();
    let notification = web_sys::Notification::new("Trading Bot Dashboard");
    
    if let Ok(n) = notification {
        // Check permission
        if web_sys::Notification::permission() != "granted" {
            let _ = web_sys::Notification::request_permission();
        }
    }
}

// Send desktop notification
pub fn send_notification(title: &str, body: &str, icon: Option<&str>) {
    if web_sys::Notification::permission() == "granted" {
        let mut options = web_sys::NotificationOptions::new();
        options.body(body);
        
        if let Some(i) = icon {
            options.icon(i);
        }
        
        let _ = web_sys::Notification::new_with_options(title, &options);
    }
}

// Initialize WebSocket connection with notification support
pub fn init_websocket<F>(on_message: F) -> WsClient
where
    F: Fn(WsMessage) + 'static,
{
    // Request notification permission on first connect
    request_notification_permission();
    
    let client = WsClient::new();
    client.on_message(on_message);
    client.on_status_change(|status| {
        match status {
            ConnectionStatus::Connected => {
                web_sys::console::log_1(&"WebSocket: Connected".into());
            }
            ConnectionStatus::Disconnected => {
                web_sys::console::warn_1(&"WebSocket: Disconnected".into());
            }
            ConnectionStatus::Reconnecting { attempt, delay_ms } => {
                web_sys::console::warn_1(&format!("WebSocket: Reconnecting... ({}, {}ms)", attempt, delay_ms).into());
            }
            ConnectionStatus::Error => {
                web_sys::console::error_1(&"WebSocket: Error".into());
                send_notification(
                    "Connection Error",
                    "Failed to connect to trading server",
                    None,
                );
            }
            _ => {}
        }
    });
    
    client.connect();
    client
}
