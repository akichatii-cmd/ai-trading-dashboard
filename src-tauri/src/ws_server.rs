use crate::api::*;
use crate::models::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc, RwLock};
use tokio::time::{interval, Duration, timeout};
use tokio_tungstenite::{
    accept_async,
    tungstenite::protocol::{frame::coding::CloseCode, CloseFrame, Message},
    WebSocketStream,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, warn};

const WS_PORT: u16 = 8081;
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(60);
const RECONNECT_DELAY: Duration = Duration::from_secs(5);

/// Типы WebSocket сообщений
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WsMessage {
    /// Обновление цены
    #[serde(rename = "price.update")]
    PriceUpdate { symbol: String, price: f64, ts: u64 },
    
    /// Обновление позиции (PnL)
    #[serde(rename = "position.update")]
    PositionUpdate { id: String, unrealized_pnl: f64, current_price: f64 },
    
    /// Новый сигнал
    #[serde(rename = "signal.new")]
    SignalGenerated { signal: TradeSignal },
    
    /// Сигнал устарел
    #[serde(rename = "signal.stale")]
    SignalStale { symbol: String, age_seconds: u64 },
    
    /// Исполнение ордера
    #[serde(rename = "order.fill")]
    OrderFilled { order_id: String, fill_price: f64, filled_qty: u32 },
    
    /// Обновление ордера
    #[serde(rename = "order.update")]
    OrderUpdate { order: Order },
    
    /// Алерт риска
    #[serde(rename = "risk.alert")]
    RiskAlert { level: String, message: String },
    
    /// Статус системы
    #[serde(rename = "system.status")]
    SystemStatus { mode: String, running: bool, health: String },
    
    /// Лог
    #[serde(rename = "log.append")]
    LogAppend { level: String, message: String, source: String, ts: u64 },
    
    /// Пинг (heartbeat)
    #[serde(rename = "ping")]
    Ping,
    
    /// Понг (heartbeat response)
    #[serde(rename = "pong")]
    Pong,
    
    /// Подписка на каналы
    #[serde(rename = "subscribe")]
    Subscribe { channels: Vec<String> },
    
    /// Отписка от каналов
    #[serde(rename = "unsubscribe")]
    Unsubscribe { channels: Vec<String> },
}

/// Состояние подключенного клиента
struct ClientConnection {
    id: String,
    subscribed_channels: Vec<String>,
    last_pong: tokio::time::Instant,
}

/// Глобальное состояние WebSocket сервера
pub struct WsServerState {
    clients: Arc<RwLock<HashMap<String, mpsc::UnboundedSender<WsMessage>>>>,
    broadcast_tx: broadcast::Sender<WsMessage>,
}

impl WsServerState {
    pub fn new() -> Self {
        let (broadcast_tx, _) = broadcast::channel(1024);
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            broadcast_tx,
        }
    }
    
    /// Отправить сообщение всем подписанным клиентам
    pub async fn broadcast(&self, msg: WsMessage) {
        let clients = self.clients.read().await;
        let mut disconnected = Vec::new();
        
        for (client_id, tx) in clients.iter() {
            if tx.send(msg.clone()).is_err() {
                disconnected.push(client_id.clone());
            }
        }
        drop(clients);
        
        // Удалить отключившихся клиентов
        if !disconnected.is_empty() {
            let mut clients = self.clients.write().await;
            for id in disconnected {
                clients.remove(&id);
                info!("Removed disconnected client: {}", id);
            }
        }
    }
    
    /// Получить broadcast receiver
    pub fn subscribe(&self) -> broadcast::Receiver<WsMessage> {
        self.broadcast_tx.subscribe()
    }
}

/// Запустить WebSocket сервер
pub async fn run_ws_server(state: Arc<WsServerState>) -> anyhow::Result<()> {
    let addr = format!("127.0.0.1:{}", WS_PORT);
    let listener = TcpListener::bind(&addr).await?;
    
    info!("WebSocket server listening on ws://{}", addr);
    add_log(LogLevel::Info, &format!("WebSocket server started on port {}", WS_PORT), "ws_server").await;
    
    // Запускаем задачу для broadcast из глобального канала
    let broadcast_state = state.clone();
    tokio::spawn(async move {
        let mut rx = broadcast_state.subscribe();
        loop {
            match rx.recv().await {
                Ok(msg) => {
                    broadcast_state.broadcast(msg).await;
                }
                Err(broadcast::error::RecvError::Lagged(_)) => {
                    warn!("Broadcast channel lagged");
                }
                Err(broadcast::error::RecvError::Closed) => {
                    error!("Broadcast channel closed");
                    break;
                }
            }
        }
    });
    
    // Принимаем входящие соединения
    while let Ok((stream, addr)) = listener.accept().await {
        let state = state.clone();
        
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, addr.to_string(), state).await {
                error!("WebSocket connection error: {}", e);
            }
        });
    }
    
    Ok(())
}

/// Обработать одно WebSocket соединение
async fn handle_connection(
    stream: TcpStream,
    addr: String,
    state: Arc<WsServerState>,
) -> anyhow::Result<()> {
    let ws_stream = accept_async(stream).await?;
    let client_id = format!("{}-{}", addr, uuid::Uuid::new_v4().to_string().split('-').next().unwrap_or(""));
    
    info!("New WebSocket connection: {}", client_id);
    add_log(LogLevel::Info, &format!("Client connected: {}", client_id), "ws_server").await;
    
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<WsMessage>();
    
    // Добавляем клиента в список
    {
        let mut clients = state.clients.write().await;
        clients.insert(client_id.clone(), tx);
    }
    
    let mut heartbeat_interval = interval(HEARTBEAT_INTERVAL);
    let mut last_pong = tokio::time::Instant::now();
    let mut subscribed_channels: Vec<String> = vec!["prices".to_string(), "positions".to_string(), "orders".to_string(), "signals".to_string(), "logs".to_string()];
    
    loop {
        tokio::select! {
            // Входящие сообщения от клиента
            msg = ws_receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<WsMessage>(&text) {
                            Ok(WsMessage::Ping) => {
                                ws_sender.send(Message::Text(
                                    serde_json::to_string(&WsMessage::Pong).unwrap()
                                )).await?;
                            }
                            Ok(WsMessage::Pong) => {
                                last_pong = tokio::time::Instant::now();
                            }
                            Ok(WsMessage::Subscribe { channels }) => {
                                subscribed_channels = channels;
                                info!("Client {} subscribed to: {:?}", client_id, subscribed_channels);
                            }
                            Ok(WsMessage::Unsubscribe { channels }) => {
                                subscribed_channels.retain(|c| !channels.contains(c));
                            }
                            Ok(other) => {
                                debug!("Received from {}: {:?}", client_id, other);
                            }
                            Err(e) => {
                                warn!("Failed to parse message from {}: {}", client_id, e);
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) => {
                        info!("Client {} closed connection", client_id);
                        break;
                    }
                    Some(Err(e)) => {
                        error!("WebSocket error from {}: {}", client_id, e);
                        break;
                    }
                    None => {
                        info!("Client {} disconnected", client_id);
                        break;
                    }
                    _ => {}
                }
            }
            
            // Исходящие сообщения (broadcast)
            Some(msg) = rx.recv() => {
                // Проверяем, подписан ли клиент на этот канал
                let should_send = match &msg {
                    WsMessage::PriceUpdate { .. } => subscribed_channels.contains(&"prices".to_string()),
                    WsMessage::PositionUpdate { .. } => subscribed_channels.contains(&"positions".to_string()),
                    WsMessage::SignalGenerated { .. } => subscribed_channels.contains(&"signals".to_string()),
                    WsMessage::OrderFilled { .. } | WsMessage::OrderUpdate { .. } => subscribed_channels.contains(&"orders".to_string()),
                    WsMessage::RiskAlert { .. } => subscribed_channels.contains(&"risks".to_string()),
                    WsMessage::LogAppend { .. } => subscribed_channels.contains(&"logs".to_string()),
                    WsMessage::SystemStatus { .. } => true, // Всегда отправляем
                    _ => true,
                };
                
                if should_send {
                    let json = serde_json::to_string(&msg)?;
                    if ws_sender.send(Message::Text(json)).await.is_err() {
                        break;
                    }
                }
            }
            
            // Heartbeat
            _ = heartbeat_interval.tick() => {
                // Проверяем timeout
                if last_pong.elapsed() > HEARTBEAT_TIMEOUT {
                    warn!("Client {} heartbeat timeout", client_id);
                    break;
                }
                
                // Отправляем ping
                let ping = serde_json::to_string(&WsMessage::Ping)?;
                if ws_sender.send(Message::Text(ping)).await.is_err() {
                    break;
                }
            }
        }
    }
    
    // Удаляем клиента при отключении
    {
        let mut clients = state.clients.write().await;
        clients.remove(&client_id);
    }
    
    add_log(LogLevel::Info, &format!("Client disconnected: {}", client_id), "ws_server").await;
    Ok(())
}

/// Периодическая рассылка mock данных (для демо)
pub async fn spawn_mock_data_feed(state: Arc<WsServerState>) {
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(1));
        let symbols = vec![("SBER", 250.0), ("GAZP", 165.0), ("LKOH", 6500.0), ("YNDX", 3800.0), ("TCSG", 2800.0)];
        let mut prices: HashMap<String, f64> = symbols.iter().map(|(s, p)| (s.to_string(), *p)).collect();
        
        loop {
            ticker.tick().await;
            
            // Генерируем обновления цен
            for (symbol, base_price) in &symbols {
                let current = prices.get(*symbol).unwrap_or(base_price);
                let change = (rand::random::<f64>() - 0.5) * 0.01; // ±0.5%
                let new_price = current * (1.0 + change);
                prices.insert(symbol.to_string(), new_price);
                
                let msg = WsMessage::PriceUpdate {
                    symbol: symbol.to_string(),
                    price: new_price,
                    ts: chrono::Utc::now().timestamp_millis() as u64,
                };
                
                // Отправляем в broadcast канал
                let _ = state.broadcast_tx.send(msg);
                
                // Обновляем цену в API
                if let Ok(price_dec) = rust_decimal::Decimal::try_from(new_price) {
                    update_price(symbol, price_dec).await;
                }
            }
        }
    });
}

/// Отправить сигнал всем подключенным клиентам
pub async fn broadcast_signal(state: Arc<WsServerState>, signal: TradeSignal) {
    let msg = WsMessage::SignalGenerated { signal };
    let _ = state.broadcast_tx.send(msg);
}

/// Отправить алерт риска
pub async fn broadcast_risk_alert(state: Arc<WsServerState>, level: &str, message: &str) {
    let msg = WsMessage::RiskAlert {
        level: level.to_string(),
        message: message.to_string(),
    };
    let _ = state.broadcast_tx.send(msg);
}

/// Отправить обновление ордера
pub async fn broadcast_order_update(state: Arc<WsServerState>, order: Order) {
    let msg = WsMessage::OrderUpdate { order };
    let _ = state.broadcast_tx.send(msg);
}

/// Отправить лог
pub async fn broadcast_log(state: Arc<WsServerState>, level: LogLevel, message: &str, source: &str) {
    let level_str = match level {
        LogLevel::Info => "INFO",
        LogLevel::Warn => "WARN",
        LogLevel::Error => "ERROR",
        LogLevel::Debug => "DEBUG",
    };
    
    let msg = WsMessage::LogAppend {
        level: level_str.to_string(),
        message: message.to_string(),
        source: source.to_string(),
        ts: chrono::Utc::now().timestamp_millis() as u64,
    };
    let _ = state.broadcast_tx.send(msg);
}
