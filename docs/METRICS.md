# Trading Metrics Reference

Key metrics to track during paper trading and live trading.

## Performance Metrics

### Win Rate

```
Win Rate = Winning Trades / Total Trades * 100%
```

**Targets**:
- Minimum acceptable: 40%
- Good: 50%+
- Excellent: 60%+

### Profit Factor

```
Profit Factor = Gross Profit / Gross Loss
```

**Targets**:
- < 1.0: Losing strategy
- 1.0-1.2: Marginal
- 1.2-1.5: Good
- 1.5+: Excellent

### Sharpe Ratio

```
Sharpe Ratio = (Average Return - Risk-Free Rate) / Standard Deviation of Returns
```

**Targets**:
- < 1.0: Sub-optimal
- 1.0-1.5: Good
- 1.5-2.0: Very Good
- 2.0+: Excellent

### Average Win/Loss Ratio

```
R:R Ratio = Average Win / Average Loss
```

**Targets**:
- Minimum: 1.5:1
- Good: 2:1
- Excellent: 3:1+

## Risk Metrics

### Maximum Drawdown

```
Max Drawdown = (Peak Value - Trough Value) / Peak Value * 100%
```

**Limits**:
- Warning: 5%
- Critical: 8%
- Trading Halt: 10%

### Daily Loss Limit

```
Daily Loss % = (Start Balance - Current Balance) / Start Balance * 100%
```

**Limit**: 2% (configurable)

### Risk per Trade

```
Risk % = (Entry - Stop Loss) * Quantity / Account Balance * 100%
```

**Target**: 1-2% per trade

## Calculations in Code

### Rust Implementation

```rust
pub struct TradingMetrics {
    pub total_trades: u32,
    pub winning_trades: u32,
    pub losing_trades: u32,
    pub gross_profit: Decimal,
    pub gross_loss: Decimal,
    pub max_drawdown: Decimal,
}

impl TradingMetrics {
    pub fn win_rate(&self) -> f64 {
        if self.total_trades == 0 {
            return 0.0;
        }
        self.winning_trades as f64 / self.total_trades as f64 * 100.0
    }
    
    pub fn profit_factor(&self) -> f64 {
        if self.gross_loss == Decimal::ZERO {
            return f64::INFINITY;
        }
        (self.gross_profit / self.gross_loss).to_f64().unwrap_or(0.0)
    }
    
    pub fn sharpe_ratio(&self, returns: &[f64]) -> f64 {
        if returns.len() < 2 {
            return 0.0;
        }
        
        let avg_return = returns.iter().sum::<f64>() / returns.len() as f64;
        let variance = returns.iter()
            .map(|r| (r - avg_return).powi(2))
            .sum::<f64>() / (returns.len() - 1) as f64;
        let std_dev = variance.sqrt();
        
        if std_dev == 0.0 {
            return 0.0;
        }
        
        // Assuming risk-free rate of 0 for simplicity
        avg_return / std_dev
    }
}
```

### JavaScript/TypeScript

```typescript
interface Trade {
    entryPrice: number;
    exitPrice: number;
    quantity: number;
    side: 'buy' | 'sell';
}

function calculateMetrics(trades: Trade[]) {
    const winningTrades = trades.filter(t => 
        (t.side === 'buy' && t.exitPrice > t.entryPrice) ||
        (t.side === 'sell' && t.exitPrice < t.entryPrice)
    );
    
    const losingTrades = trades.filter(t => 
        (t.side === 'buy' && t.exitPrice < t.entryPrice) ||
        (t.side === 'sell' && t.exitPrice > t.entryPrice)
    );
    
    const winRate = (winningTrades.length / trades.length) * 100;
    
    const grossProfit = winningTrades.reduce((sum, t) => 
        sum + Math.abs(t.exitPrice - t.entryPrice) * t.quantity, 0
    );
    
    const grossLoss = losingTrades.reduce((sum, t) => 
        sum + Math.abs(t.exitPrice - t.entryPrice) * t.quantity, 0
    );
    
    const profitFactor = grossLoss > 0 ? grossProfit / grossLoss : Infinity;
    
    return { winRate, profitFactor, grossProfit, grossLoss };
}
```

## Position Sizing

### Fixed Fractional

```rust
pub fn calculate_position_size(
    account_balance: Decimal,
    risk_percent: Decimal,
    entry_price: Decimal,
    stop_loss: Decimal,
) -> u32 {
    let risk_amount = account_balance * risk_percent / Decimal::from(100);
    let risk_per_share = (entry_price - stop_loss).abs();
    
    if risk_per_share == Decimal::ZERO {
        return 0;
    }
    
    let shares = risk_amount / risk_per_share;
    shares.floor().to_u32().unwrap_or(0)
}
```

### Kelly Criterion

```
f* = (bp - q) / b

where:
  f* = optimal position size
  b = average win / average loss
  p = win probability
  q = 1 - p (loss probability)
```

```rust
pub fn kelly_criterion(win_rate: f64, avg_win: f64, avg_loss: f64) -> f64 {
    let b = avg_win / avg_loss;
    let p = win_rate;
    let q = 1.0 - p;
    
    (b * p - q) / b
}
```

**Note**: Use Half-Kelly or Quarter-Kelly for safety.

## Slippage Calculation

```
Slippage % = |Actual Fill Price - Expected Price| / Expected Price * 100
```

**Targets**:
- Market orders: < 0.1%
- Limit orders: 0% (should fill at limit)

## Equity Curve

Track cumulative P&L over time:

```rust
pub fn generate_equity_curve(trades: &[Trade]) -> Vec<(DateTime, Decimal)> {
    let mut equity = Decimal::from(10000); // Starting balance
    let mut curve = vec![(trades[0].entry_time, equity)];
    
    for trade in trades {
        equity += trade.pnl;
        curve.push((trade.exit_time, equity));
    }
    
    curve
}
```

## Benchmarking

Compare against buy-and-hold:

```
Alpha = Strategy Return - Benchmark Return
```

**Benchmarks**:
- S&P 500 (SPY)
- NASDAQ (QQQ)
- Total Market (VTI)

---

*Track these metrics daily during paper trading to evaluate strategy performance.*
