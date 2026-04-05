# API Documentation

## Tauri Commands

All commands are available via `window.__TAURI__.invoke()` or `tauri::invoke()`.

### Bot Control

#### `get_bot_status`
Returns current trading bot status.

**Parameters:** None

**Returns:**
```rust
struct BotStatus {
    running: bool,
    mode: String,      // "paper" | "live" | "backtest"
    uptime_secs: u64,
    connected: bool,
}
```

**Example:**
```javascript
const status = await invoke('get_bot_status');
console.log(status.running); // true
```

---

#### `start_trading`
Starts trading in specified mode.

**Parameters:**
```rust
struct StartTradingArgs {
    mode: String,      // "paper" or "live"
}
```

**Returns:** `Result<(), String>`

**Errors:**
- `"Already running"` - Bot is already active
- `"Invalid mode"` - Mode not "paper" or "live"
- `"API token not configured"` - Missing TINKOFF_TOKEN

**Example:**
```javascript
await invoke('start_trading', { mode: 'paper' });
```

---

#### `stop_trading`
Stops trading gracefully.

**Parameters:** None

**Returns:** `Result<(), String>`

---

#### `emergency_stop`
Closes all positions and stops trading immediately.

**Parameters:** None

**Returns:** `Result<StopResult, String>`

```rust
struct StopResult {
    positions_closed: u32,
    orders_cancelled: u32,
    total_pnl: f64,
}
```

---

### Trading Operations

#### `place_order`
Places a new order.

**Parameters:**
```rust
struct PlaceOrderArgs {
    symbol: String,           // "AAPL", "TSLA", etc.
    side: String,             // "buy" | "sell"
    order_type: String,       // "market" | "limit" | "stop"
    quantity: u32,            // Number of shares/contracts
    price: Option<String>,    // Required for limit/stop
}
```

**Returns:**
```rust
struct OrderResult {
    order_id: String,
    status: String,           // "filled" | "pending" | "rejected"
    filled_quantity: u32,
    avg_price: Option<String>,
}
```

**Example:**
```javascript
const result = await invoke('place_order', {
    symbol: 'AAPL',
    side: 'buy',
    order_type: 'market',
    quantity: 10
});
```

---

#### `close_position`
Closes an open position.

**Parameters:**
```rust
struct ClosePositionArgs {
    position_id: String,
}
```

**Returns:** `Result<CloseResult, String>`

```rust
struct CloseResult {
    pnl: f64,
    exit_price: f64,
    timestamp: i64,
}
```

---

#### `modify_position_sl_tp`
Updates stop-loss and/or take-profit for a position.

**Parameters:**
```rust
struct ModifySlTpArgs {
    position_id: String,
    new_sl: Option<f64>,      // null to keep current
    new_tp: Option<f64>,      // null to keep current
}
```

**Returns:** `Result<(), String>`

**Example:**
```javascript
// Update SL only
await invoke('modify_position_sl_tp', {
    positionId: 'pos_123',
    newSl: 145.50,
    newTp: null
});

// Update both
await invoke('modify_position_sl_tp', {
    positionId: 'pos_123',
    newSl: 145.50,
    newTp: 165.00
});
```

---

#### `cancel_order`
Cancels a pending order.

**Parameters:**
```rust
struct CancelOrderArgs {
    order_id: String,
}
```

**Returns:** `Result<(), String>`

---

### Data Queries

#### `get_positions`
Returns all open positions.

**Parameters:** None

**Returns:**
```rust
Vec<Position>

struct Position {
    id: String,
    symbol: String,
    side: String,             // "long" | "short"
    quantity: u32,
    entry_price: f64,
    current_price: f64,
    stop_loss: Option<f64>,
    take_profit: Option<f64>,
    unrealized_pnl: f64,
    unrealized_pnl_pct: f64,
    opened_at: i64,
}
```

---

#### `get_orders`
Returns order history.

**Parameters:**
```rust
struct GetOrdersArgs {
    status: Option<String>,   // "filled" | "pending" | "cancelled" | null (all)
    limit: Option<u32>,       // default: 100
}
```

**Returns:** `Vec<Order>`

---

#### `get_historical_data`
Returns candlestick data for charting.

**Parameters:**
```rust
struct GetHistoricalDataArgs {
    symbol: String,
    timeframe: String,        // "1" | "5" | "15" | "30" | "60" | "240" | "D"
    from: i64,                // Unix timestamp (seconds)
    to: i64,                  // Unix timestamp (seconds)
}
```

**Returns:**
```rust
Vec<CandleData>

struct CandleData {
    timestamp: i64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: u64,
}
```

**Example:**
```javascript
const candles = await invoke('get_historical_data', {
    symbol: 'AAPL',
    timeframe: '15',
    from: 1704067200,
    to: 1706659200
});
```

---

#### `get_account_summary`
Returns account balance and statistics.

**Parameters:** None

**Returns:**
```rust
struct AccountSummary {
    balance: f64,
    equity: f64,
    margin_used: f64,
    margin_available: f64,
    daily_pnl: f64,
    daily_pnl_pct: f64,
    total_pnl: f64,
    open_positions_count: u32,
    open_orders_count: u32,
}
```

---

#### `get_trade_history`
Returns closed trades.

**Parameters:**
```rust
struct GetTradeHistoryArgs {
    from: Option<i64>,        // Unix timestamp
    to: Option<i64>,
    limit: Option<u32>,
}
```

**Returns:** `Vec<TradeRecord>`

---

### Settings

#### `get_settings`
Returns application settings.

**Parameters:** None

**Returns:**
```rust
struct Settings {
    default_quantity: u32,
    risk_per_trade_pct: f64,
    max_positions: u32,
    max_daily_loss_pct: f64,
    enable_sounds: bool,
    theme: String,            // "dark" | "light"
}
```

---

#### `update_settings`
Updates application settings.

**Parameters:** `Settings`

**Returns:** `Result<(), String>`

---

## WebSocket Protocol

### Connection

```javascript
const ws = new WebSocket('ws://127.0.0.1:8081');
```

### Client → Server Messages

#### Subscribe to channel
```json
{
    "action": "subscribe",
    "channel": "prices",
    "symbol": "AAPL"
}
```

#### Unsubscribe
```json
{
    "action": "unsubscribe",
    "channel": "prices"
}
```

#### Ping (keepalive)
```json
{
    "action": "ping"
}
```

### Server → Client Messages

#### Price Update
```json
{
    "type": "PriceUpdate",
    "symbol": "AAPL",
    "bid": 175.50,
    "ask": 175.55,
    "last": 175.52,
    "timestamp": 1704067200
}
```

#### Position Update
```json
{
    "type": "PositionUpdate",
    "positions": [
        {
            "id": "pos_123",
            "symbol": "AAPL",
            "side": "long",
            "quantity": 10,
            "entry_price": 170.00,
            "current_price": 175.52,
            "stop_loss": 165.00,
            "take_profit": 180.00,
            "unrealized_pnl": 55.20,
            "unrealized_pnl_pct": 3.25
        }
    ]
}
```

#### Signal Generated
```json
{
    "type": "SignalGenerated",
    "signal": {
        "id": "sig_456",
        "symbol": "TSLA",
        "action": "buy",
        "confidence": 0.87,
        "entry_price": 240.50,
        "stop_loss": 235.00,
        "take_profit": 255.00,
        "risk_reward": 2.5,
        "timestamp": 1704067200
    }
}
```

#### Order Executed
```json
{
    "type": "OrderExecuted",
    "order": {
        "id": "ord_789",
        "symbol": "AAPL",
        "side": "buy",
        "status": "filled",
        "filled_quantity": 10,
        "avg_price": 175.50
    }
}
```

#### Error
```json
{
    "type": "Error",
    "code": 1001,
    "message": "Insufficient funds for order"
}
```

### Reconnection

The client should implement exponential backoff:

```javascript
let attempt = 0;
const maxDelay = 30000; // 30 seconds

function connect() {
    const ws = new WebSocket('ws://127.0.0.1:8081');
    
    ws.onclose = () => {
        const delay = Math.min(1000 * Math.pow(2, attempt), maxDelay);
        attempt++;
        setTimeout(connect, delay);
    };
    
    ws.onopen = () => {
        attempt = 0;
    };
}
```

## Error Codes

| Code | Meaning | Description |
|------|---------|-------------|
| 1000 | Internal error | Unexpected server error |
| 1001 | Insufficient funds | Not enough balance for order |
| 1002 | Invalid symbol | Symbol not found or not tradeable |
| 1003 | Market closed | Trading hours restriction |
| 1004 | Rate limited | Too many requests |
| 1005 | Validation error | Invalid parameters |
| 1006 | Connection error | WebSocket connection issue |
| 1007 | Not found | Position or order not found |
| 1008 | Already exists | Duplicate order or operation |

## Rate Limits

- WebSocket: 100 messages/second per connection
- REST commands: 10 requests/second
- Historical data: 1 request/minute per symbol

## TypeScript Definitions

```typescript
interface BotStatus {
    running: boolean;
    mode: 'paper' | 'live' | 'backtest';
    uptime_secs: number;
    connected: boolean;
}

interface PlaceOrderArgs {
    symbol: string;
    side: 'buy' | 'sell';
    order_type: 'market' | 'limit' | 'stop';
    quantity: number;
    price?: string;
}

interface Position {
    id: string;
    symbol: string;
    side: 'long' | 'short';
    quantity: number;
    entry_price: number;
    current_price: number;
    stop_loss?: number;
    take_profit?: number;
    unrealized_pnl: number;
    unrealized_pnl_pct: number;
    opened_at: number;
}

interface Signal {
    id: string;
    symbol: string;
    action: 'buy' | 'sell' | 'hold';
    confidence: number;
    entry_price: number;
    stop_loss: number;
    take_profit: number;
    risk_reward: number;
    timestamp: number;
}

type WsMessage = 
    | { type: 'PriceUpdate'; symbol: string; bid: number; ask: number; last: number; timestamp: number }
    | { type: 'PositionUpdate'; positions: Position[] }
    | { type: 'SignalGenerated'; signal: Signal }
    | { type: 'OrderExecuted'; order: any }
    | { type: 'Error'; code: number; message: string };
```

## Examples

### Complete Trading Flow

```javascript
// 1. Start trading
await invoke('start_trading', { mode: 'paper' });

// 2. Connect WebSocket
const ws = new WebSocket('ws://127.0.0.1:8081');
ws.onmessage = (e) => {
    const msg = JSON.parse(e.data);
    if (msg.type === 'SignalGenerated') {
        // 3. Execute signal
        executeSignal(msg.signal);
    }
};

// 4. Place order from signal
async function executeSignal(signal) {
    const order = await invoke('place_order', {
        symbol: signal.symbol,
        side: signal.action,
        order_type: 'market',
        quantity: calculatePositionSize(signal)
    });
    
    // 5. Set SL/TP
    await invoke('modify_position_sl_tp', {
        positionId: order.position_id,
        newSl: signal.stop_loss,
        newTp: signal.take_profit
    });
}
```

---

*Last updated: 2024*
