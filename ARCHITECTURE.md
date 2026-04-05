# Architecture Documentation

## System Overview

The AI Trading Bot Dashboard is a desktop application built with Tauri, using a Rust backend and Leptos (WASM) frontend. It communicates with trading APIs via WebSocket and REST.

```
┌─────────────────────────────────────────────────────────────────────┐
│                         User Interface                               │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  Leptos (WASM)                                               │   │
│  │  ┌─────────────┐  ┌──────────────┐  ┌─────────────────────┐ │   │
│  │  │ Components  │  │   WebSocket  │  │   Tauri Bridge      │ │   │
│  │  │  - Chart    │◄─┤    Client    │  │  (Commands/Events)  │ │   │
│  │  │  - Panels   │  │  (port 8081) │  │                     │ │   │
│  │  │  - Terminal │  └──────────────┘  └─────────────────────┘ │   │
│  │  └─────────────┘              │                    │         │   │
│  └───────────────────────────────┼────────────────────┼─────────┘   │
└──────────────────────────────────┼────────────────────┼─────────────┘
                                   │                    │
                                   ▼                    ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      Tauri Runtime (Rust)                            │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  Backend                                                     │   │
│  │  ┌─────────────┐  ┌──────────────┐  ┌─────────────────────┐ │   │
│  │  │   State     │  │   WebSocket  │  │    Tauri Commands   │ │   │
│  │  │ Management  │◄─┤    Server    │◄─┤                     │ │   │
│  │  │  (RwLock)   │  │  (broadcast) │  │  • place_order      │ │   │
│  │  └─────────────┘  └──────────────┘  │  • close_position   │ │   │
│  │                                     │  • modify_sl_tp     │ │   │
│  │  ┌─────────────┐                    │  • get_positions    │ │   │
│  │  │   Tinkoff   │◄───────────────────┘  • emergency_stop   │ │   │
│  │  │    API      │                       • get_history       │ │   │
│  │  │  (gRPC)     │                                            │   │
│  │  └─────────────┘                                            │   │
│  └─────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

## Component Details

### Frontend (ui/)

#### WebSocket Client (`websocket.rs`)

```rust
pub struct WebSocketClient {
    url: String,
    reconnect_attempt: RwSignal<u32>,
    status: RwSignal<WsStatus>,
    on_message: Box<dyn Fn(WsMessage)>,
}

impl WebSocketClient {
    pub fn connect(&self) -> Result<(), JsValue>
    pub fn reconnect_with_backoff(&mut self)
}
```

**Features:**
- Exponential backoff: 1s → 2s → 4s → ... → 30s max
- Auto-reconnect on connection loss
- Message deserialization via serde
- Status callbacks (Connecting/Connected/Error)

#### State Management

Uses Leptos reactive signals:

```rust
// Global signals
pub static PRICES: RwSignal<HashMap<String, PriceData>> = RwSignal::new(HashMap::new());
pub static POSITIONS: RwSignal<Vec<Position>> = RwSignal::new(Vec::new());
pub static LATEST_SIGNAL: RwSignal<Option<Signal>> = RwSignal::new(None);
pub static WS_STATUS: RwSignal<WsStatus> = RwSignal::new(WsStatus::Disconnected);
```

#### Components

```
app.rs
├── LeftPanel (Watchlist, Positions, Balance)
│   ├── WatchlistItem
│   ├── PositionCard
│   └── AccountSummary
├── CenterPanel (Chart, Order Entry)
│   ├── TradingViewChart
│   ├── OrderModal
│   └── PriceDisplay
├── RightPanel (Signals, Risk, Quick Trade)
│   ├── SignalCard
│   ├── RiskDashboard
│   └── QuickOrderButtons
└── Terminal (Logs, System Messages)
    ├── LogEntry
    └── FilterControls
```

### Backend (src-tauri/)

#### WebSocket Server (`ws_server.rs`)

```rust
pub async fn run_ws_server() {
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    let connections = Connections::default();
    
    // Spawn tasks
    tokio::spawn(generate_mock_data(connections.clone()));
    tokio::spawn(heartbeat_checker(connections.clone()));
    
    // Accept connections
    while let Ok((stream, _)) = listener.accept().await {
        handle_connection(stream, connections.clone()).await;
    }
}
```

**Message Types:**
```rust
pub enum WsMessage {
    PriceUpdate { symbol: String, bid: f64, ask: f64, timestamp: i64 },
    PositionUpdate { positions: Vec<Position> },
    SignalGenerated { signal: Signal },
    OrderExecuted { order_id: String, status: OrderStatus },
    Error { code: u16, message: String },
}
```

#### State Management (`state.rs`)

```rust
pub struct TradingState {
    pub positions: RwLock<Vec<Position>>,
    pub orders: RwLock<Vec<Order>>,
    pub balance: RwLock<Balance>,
    pub config: RwLock<TradingConfig>,
    pub daily_stats: RwLock<DailyStats>,
}

lazy_static! {
    pub static ref STATE: TradingState = TradingState::new();
}
```

#### Tauri Commands (`api.rs`)

Command flow:
```
Frontend invoke
    ↓
Tauri command handler
    ↓
State update
    ↓
WebSocket broadcast
    ↓
All clients receive update
```

## Data Flow

### Price Update Flow

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ Price Source │────►│ WS Server    │────►│ All Clients  │
│ (Mock/API)   │     │ (broadcast)  │     │ (UI update)  │
└──────────────┘     └──────────────┘     └──────────────┘
                           │
                           ▼
                    ┌──────────────┐
                    │ Chart Update │
                    │ (TV real-time)│
                    └──────────────┘
```

### Order Execution Flow

```
┌──────────┐    ┌──────────────┐    ┌──────────────┐    ┌──────────┐
│  User    │───►│ Tauri Cmd    │───►│ Tinkoff API  │───►│ Exchange │
│ Clicks   │    │ validate()   │    │ place_order()│    │          │
└──────────┘    └──────────────┘    └──────────────┘    └──────────┘
                       │
                       ▼
                ┌──────────────┐
                │ State Update │
                │ WS Broadcast │
                └──────────────┘
```

### SL/TP Modification Flow

```
┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│ Drag on Chart│───►│ JS Event     │───►│ Tauri invoke │
│ (TradingView)│    │ priceLineMoved│    │ modify_sl_tp │
└──────────────┘    └──────────────┘    └──────────────┘
                                               │
                                               ▼
                                        ┌──────────────┐
                                        │ Update State │
                                        │ Broadcast    │
                                        └──────────────┘
```

## Security Considerations

### API Token Handling

```rust
// .env file (gitignored)
TINKOFF_TOKEN=your_token_here

// Code
use dotenv::dotenv;
use std::env;

pub fn get_tinkoff_token() -> String {
    dotenv().ok();
    env::var("TINKOFF_TOKEN").expect("TINKOFF_TOKEN not set")
}
```

### Input Validation

```rust
#[tauri::command]
pub async fn place_order(args: PlaceOrderArgs) -> Result<Order, String> {
    // Validate symbol
    if !is_valid_symbol(&args.symbol) {
        return Err("Invalid symbol".into());
    }
    
    // Validate quantity
    if args.quantity == 0 {
        return Err("Quantity must be > 0".into());
    }
    
    // Validate price for limit orders
    if args.order_type == "limit" && args.price.is_none() {
        return Err("Limit order requires price".into());
    }
    
    // ... execute
}
```

## Performance Optimizations

### WebSocket Broadcasting

```rust
// Use tokio::sync::broadcast for efficient fan-out
let (tx, _rx) = broadcast::channel::<WsMessage>(100);

// Clone sender for each connection
let tx_clone = tx.clone();
tokio::spawn(async move {
    while let Ok(msg) = rx.recv().await {
        if socket.send(msg).await.is_err() {
            break; // Client disconnected
        }
    }
});
```

### Frontend Rendering

```rust
// Use For component for lists
view! {
    <For
        each=positions
        key=|p| p.id.clone()
        view=|position| view! { <PositionCard position=position /> }
    />
}
```

### Chart Performance

- Use `subscribeBars` for real-time updates
- Debounce rapid price changes
- Limit visible data points (last 1000 candles)

## Error Handling

### Backend

```rust
pub type Result<T> = std::result::Result<T, TradingError>;

#[derive(Debug, thiserror::Error)]
pub enum TradingError {
    #[error("API error: {0}")]
    Api(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Insufficient funds")]
    InsufficientFunds,
}

impl serde::Serialize for TradingError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
```

### Frontend

```rust
pub async fn place_order(args: PlaceOrderArgs) -> Result<Order, String> {
    match tauri::invoke("place_order", &args).await {
        Ok(order) => Ok(order),
        Err(e) => {
            log::error!("Order failed: {}", e);
            Err(format!("Failed to place order: {}", e))
        }
    }
}
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_position_pnl_calculation() {
        let pos = Position::new("AAPL", 100.0, 10);
        assert_eq!(pos.unrealized_pnl(110.0), 100.0);
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_websocket_connection() {
    let server = spawn_test_server().await;
    let client = WebSocketClient::connect("ws://localhost:8081").await;
    
    assert!(client.is_connected());
    
    // Test message roundtrip
    client.send(WsMessage::Ping).await;
    let response = client.recv().await;
    assert!(matches!(response, WsMessage::Pong));
}
```

## Deployment

### Desktop App

```bash
# Build for current platform
cargo tauri build

# Output
src-tauri/target/release/ai-trading-dashboard.exe
```

### Code Signing (Windows)

```powershell
# Sign with certificate
signtool sign /f certificate.pfx /p password `
  /tr http://timestamp.digicert.com `
  /td sha256 /fd sha256 `
  "ai-trading-dashboard.exe"
```

## Future Architecture

### Planned Improvements

1. **Plugin System**: WASM plugins for custom strategies
2. **Database**: SQLite for local trade history
3. **Cloud Sync**: Optional cloud backup of settings
4. **Multi-Monitor**: Drag charts to separate windows
5. **Mobile Companion**: React Native app for monitoring

---

*This document is a living specification. Update when architecture changes.*
