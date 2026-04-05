# Paper Trading Guide

Complete guide to testing the trading dashboard with Tinkoff Sandbox.

## 📋 Pre-Flight Checklist

### 1. Get Tinkoff Sandbox Token

1. Register at [tinkoff.ru/invest/](https://www.tinkoff.ru/invest/)
2. Go to [Invest API](https://tinkoff.github.io/investAPI/token/)
3. Generate **sandbox token** (not production!)
4. Copy token to clipboard

### 2. Configure Environment

Create `.env` file in `src-tauri/`:

```bash
# Copy template
copy .env.example .env

# Edit with your token
TINKOFF_TOKEN=your_sandbox_token_here
TINKOFF_ACCOUNT_ID=
MODE=sandbox
ENABLE_MOCK_DATA=false
ENABLE_REAL_TRADING=false
```

### 3. Verify Configuration

```bash
cd src-tauri
cargo run
```

Check logs for:
```
[INFO] Connected to Tinkoff Sandbox
[INFO] Account: 1234567890
[INFO] Balance: ₽100,000.00 (sandbox)
```

---

## 🗓️ 7-Day Paper Trading Plan

### Day 1: Connectivity Check

**Goal**: Verify everything works

**Tasks**:
- [ ] Start application
- [ ] Verify WebSocket connection (green indicator)
- [ ] Place 5 test trades (small size)
- [ ] Verify orders appear in Tinkoff mobile app
- [ ] Test emergency stop button

**Success Criteria**:
- All trades execute within 2 seconds
- No WebSocket disconnections
- Orders visible in Tinkoff app

---

### Day 2: Execution Speed

**Goal**: Validate order execution latency

**Tasks**:
- [ ] Place 10 market orders
- [ ] Measure execution time (check logs)
- [ ] Test during market open (10:00-11:00 MSK)
- [ ] Test during low volatility (14:00-15:00 MSK)

**Metrics to Track**:
| Metric | Target | Actual |
|--------|--------|--------|
| Avg execution time | < 1s | |
| Max execution time | < 3s | |
| Failed orders | 0 | |

---

### Day 3: Risk Limits Testing

**Goal**: Verify risk management works

**Tasks**:
- [ ] Open 3 positions (max limit)
- [ ] Try to open 4th position (should fail/warn)
- [ ] Simulate 5% drawdown (trade losing positions)
- [ ] Check dashboard warnings
- [ ] Test daily loss limit (if reached)

**Expected Behavior**:
- Warning at 5% drawdown (yellow)
- Critical at 8% drawdown (red, flash)
- Trading lock at 10% drawdown

---

### Day 4: Slippage Analysis

**Goal**: Measure price slippage

**Tasks**:
- [ ] Place 20 orders of varying sizes
- [ ] Record expected vs actual fill price
- [ ] Test during high volatility (news time)
- [ ] Test with limit orders vs market orders

**Slippage Formula**:
```
slippage % = |actual_price - expected_price| / expected_price * 100
```

**Target**: Average slippage < 0.1%

---

### Day 5: Strategy Tuning

**Goal**: Optimize signal parameters

**Tasks**:
- [ ] Run with current AI settings
- [ ] Record signal accuracy
- [ ] Adjust confidence threshold
- [ ] Test different risk/reward ratios
- [ ] Optimize position sizing

**Adjustments**:
```rust
// In config
min_confidence: 0.75,  // Try 0.70, 0.80
min_risk_reward: 2.0,  // Try 1.5, 2.5
risk_per_trade: 1.0,   // Try 0.5, 2.0
```

---

### Day 6: Metrics Validation

**Goal**: Collect comprehensive statistics

**Tasks**:
- [ ] Execute 20 trades
- [ ] Export trade history
- [ ] Calculate metrics:
  - Win rate
  - Average win/loss
  - Profit factor
  - Sharpe ratio
  - Max drawdown

**Target Metrics**:
| Metric | Minimum | Target |
|--------|---------|--------|
| Win rate | 40% | 50%+ |
| Profit factor | 1.2 | 1.5+ |
| Sharpe ratio | 1.0 | 1.5+ |
| Max drawdown | < 10% | < 5% |

---

### Day 7: Go/No-Go Decision

**Goal**: Decide if ready for real trading

**Checklist**:
- [ ] 90%+ system uptime
- [ ] No critical bugs found
- [ ] Risk limits work correctly
- [ ] Emergency stop < 3 seconds
- [ ] All metrics meet targets
- [ ] Comfortable with interface

**Decision Matrix**:

| Condition | Action |
|-----------|--------|
| All metrics green | ✅ Ready for small real account |
| 1-2 metrics yellow | ⚠️ Extend paper trading 3 more days |
| Any metric red | ❌ Fix issues, restart paper trading |

---

## 🔍 Daily Log Template

```markdown
## Day X - [Date]

### Trades
| Time | Symbol | Side | Entry | Exit | PnL | Notes |
|------|--------|------|-------|------|-----|-------|
| | | | | | | |

### Metrics
- Win rate: X%
- Avg slippage: X%
- System uptime: X%
- Max drawdown: X%

### Issues
- 

### Adjustments
- 

### Notes
- 
```

---

## 🎯 Key Tests to Perform

### Kill Switch Test

1. Open 3 positions
2. Click Emergency Stop
3. Time how long until all positions close
4. **Target**: < 3 seconds

### Risk Lock Test

1. Set max drawdown to 2% (for testing)
2. Open losing trades until limit hit
3. Verify trading locks automatically
4. Verify unlock procedure works

### Connection Loss Test

1. Start trading
2. Disconnect internet for 30 seconds
3. Reconnect
4. Verify:
   - Positions still tracked
   - No duplicate orders
   - Data syncs correctly

### Restart Test

1. Open positions
2. Close application
3. Reopen application
4. Verify positions restored correctly

---

## 📊 Exporting Results

### Trade History

```bash
# Export from Tinkoff API
curl -H "Authorization: Bearer $TOKEN" \
  "https://sandbox-invest-public-api.tinkoff.ru/rest/tinkoff.public.invest.api.contract.v1.OperationsService/GetOperations"
```

### Dashboard Logs

Logs location:
- Windows: `%APPDATA%/ai-trading-bot/logs/`
- Or check terminal output

---

## ⚠️ Common Paper Trading Pitfalls

1. **Not testing enough scenarios**
   - Test different market conditions
   - Test during high/low volatility

2. **Ignoring slippage**
   - Paper fills may be better than real
   - Add 0.1-0.2% slippage buffer

3. **Perfect execution assumption**
   - Real systems have delays
   - Test with slower execution

4. **Not testing failure modes**
   - Always test error conditions
   - Verify recovery works

---

## ✅ Final Go-Live Checklist

Before switching to real trading:

- [ ] 100+ paper trades completed
- [ ] All 7 days of testing passed
- [ ] Sharpe ratio > 1.0
- [ ] Max drawdown < 5%
- [ ] Win rate > 40%
- [ ] Emergency stop tested 5+ times
- [ ] Risk limits triggered and verified
- [ ] Backup plan documented
- [ ] Start with small capital (< $1000)
- [ ] Gradual scale-up plan ready

---

## 📞 Need Help?

If you encounter issues during paper trading:

1. Check [TROUBLESHOOTING.md](./TROUBLESHOOTING.md)
2. Review logs in terminal
3. Check Tinkoff API status
4. Open an issue with:
   - Trade history
   - Log output
   - Steps to reproduce
