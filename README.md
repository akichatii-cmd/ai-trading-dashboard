# 🤖 AI Trading Bot Dashboard

> Professional-grade trading interface with real-time WebSocket streaming, interactive charts, and AI-powered signal generation.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)
![Tauri](https://img.shields.io/badge/Tauri-1.5-purple.svg)
![Leptos](https://img.shields.io/badge/Leptos-0.5-green.svg)

## ✨ Features

### 📊 Real-Time Trading
- **WebSocket Streaming**: Live price updates via tokio-tungstenite (port 8081)
- **TradingView Charts**: Professional charting with Lightweight Charts™
- **Interactive SL/TP**: Drag-and-drop stop-loss and take-profit lines
- **Multiple Timeframes**: 1m, 5m, 15m, 30m, 1h, 4h, Daily

### 🎯 AI Signal System
- **Real-time Signals**: Buy/Sell/Hold recommendations with confidence scoring
- **Risk Visualization**: Animated progress bars showing risk metrics
- **One-Click Execution**: Instant order placement from signal cards
- **Signal History**: Track performance over time

### 📈 Risk Management Dashboard
- **Drawdown Monitoring**: Visual warnings at 5% (caution) and 8% (critical)
- **Daily Loss Limits**: Automatic trading lock at 2% daily loss
- **Position Limits**: Max 3 concurrent positions with visual counter
- **Kill Switch**: Emergency stop with position closure confirmation

### 🖥️ Professional Interface
- **Responsive Layout**: 300px sidebars | fluid center | collapsible terminal
- **Dark Theme**: Optimized for long trading sessions
- **Keyboard Shortcuts**: Quick actions for power users
- **Multi-Monitor Support**: 1920x1080+ optimized

## 🚀 Quick Start

### Option 1: GitHub Codespaces (Recommended) ⭐

No local setup needed! Build in the cloud:

1. **Push to GitHub**: Run `scripts/push-to-github.bat`
2. **Create Codespace**: On GitHub repo → Code → Codespaces → Create
3. **Build**: In Codespaces terminal: `cd src-tauri && cargo build --release`
4. **Download**: Get your `.exe` from the artifacts

📚 [Complete Codespaces Guide](CODESPACES_GUIDE.md)

### Option 2: Local Build

#### Prerequisites

```bash
# Rust toolchain (1.75+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js (18+)
npm install -g @tauri-apps/cli trunk

# Tailwind CSS processor
npm install -g tailwindcss

# Tauri dependencies (Windows)
# Install Visual Studio Build Tools with C++ workload
```

### Development

```bash
# Clone repository
git clone https://github.com/yourusername/ai-trading-dashboard.git
cd ai-trading-dashboard

# Install dependencies
npm install

# Terminal 1: Run WebSocket server + Tauri backend
cd src-tauri
cargo run

# Terminal 2: Run frontend dev server
cd ui
trunk serve

# Terminal 3: Watch Tailwind CSS changes
cd ui
npx tailwindcss -i ./styles/main.css -o ./styles/output.css --watch

# Or use the helper script (Windows)
./dev.ps1
```

### Production Build

```bash
# Build everything
cargo tauri build

# Output:
# src-tauri/target/release/ai-trading-dashboard.exe
```

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Tauri Application                       │
│  ┌─────────────┐  ┌──────────────┐  ┌─────────────────────┐ │
│  │   Leptos    │  │  WebSocket   │  │   Tauri Commands    │ │
│  │  Frontend   │◄─┤   Client     │  │                     │ │
│  │  (WASM)     │  │  (port 8081) │  │  • place_order      │ │
│  └─────────────┘  └──────────────┘  │  • close_position   │ │
│         │                │          │  • modify_sl_tp     │ │
│         └────────────────┘          │  • emergency_stop   │ │
│                   │                 └─────────────────────┘ │
│                   ▼                          │              │
│  ┌─────────────────────────────────┐         │              │
│  │      WebSocket Server           │◄────────┘              │
│  │  (tokio-tungstenite)            │                        │
│  │                                 │                        │
│  │  • Broadcast price updates      │                        │
│  │  • Position state management    │                        │
│  │  • Signal generation            │                        │
│  └─────────────────────────────────┘                        │
│                   │                                         │
│                   ▼                                         │
│  ┌─────────────────────────────────┐                        │
│  │     Tinkoff Invest API          │                        │
│  │      (Paper/Sandbox)            │                        │
│  └─────────────────────────────────┘                        │
└─────────────────────────────────────────────────────────────┘
```

## 📁 Project Structure

```
desktop-dashboard/
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs         # App entry point
│   │   ├── api.rs          # Tauri commands
│   │   ├── ws_server.rs    # WebSocket server
│   │   └── state.rs        # Global state management
│   ├── .env.example        # Environment template
│   └── Cargo.toml
├── ui/                     # Leptos frontend
│   ├── src/
│   │   ├── main.rs         # App root
│   │   ├── app.rs          # Main layout
│   │   ├── components/     # UI components
│   │   │   ├── left_panel.rs
│   │   │   ├── center_panel.rs
│   │   │   ├── right_panel.rs
│   │   │   └── terminal.rs
│   │   ├── websocket.rs    # WebSocket client
│   │   └── tauri_api.rs    # Tauri invoke helpers
│   ├── public/
│   │   └── tradingview.js  # Chart integration
│   ├── styles/
│   │   ├── main.css        # Tailwind source
│   │   └── output.css      # Generated styles
│   ├── index.html
│   └── Cargo.toml
├── scripts/                # Helper scripts
│   ├── fix-windows-defender.bat
│   └── start-dev.ps1
├── docs/                   # Documentation
│   ├── ARCHITECTURE.md     # System architecture
│   ├── API.md              # API reference
│   ├── PAPER_TRADING_GUIDE.md
│   ├── TROUBLESHOOTING.md
│   └── METRICS.md          # Trading metrics
├── tailwind.config.js
├── README.md
├── CONTRIBUTING.md
├── CHANGELOG.md
└── LICENSE
```

## ⚙️ Configuration

### Environment Variables

Create `.env` in `src-tauri/`:

```bash
# Tinkoff API (for paper trading)
TINKOFF_TOKEN=your_sandbox_token_here

# WebSocket Configuration
WS_PORT=8081
WS_HOST=127.0.0.1

# Trading Parameters
MAX_POSITIONS=3
MAX_DAILY_LOSS_PCT=2.0
MAX_DRAWDOWN_PCT=10.0
DEFAULT_RISK_PER_TRADE=1.0
```

### Get Tinkoff Sandbox Token

1. Register at [tinkoff.ru/invest/](https://www.tinkoff.ru/invest/)
2. Go to [Invest API](https://tinkoff.github.io/investAPI/token/)
3. Generate sandbox token
4. Add to `.env` file

📚 **See [PAPER_TRADING_GUIDE.md](docs/PAPER_TRADING_GUIDE.md) for complete setup and 7-day testing plan.**

## 🎮 Usage

### Main Interface

| Panel | Description |
|-------|-------------|
| **Left Sidebar** | Watchlist, account balance, position list |
| **Center** | TradingView chart with SL/TP lines |
| **Right Sidebar** | AI signals, risk dashboard, order panel |
| **Footer** | Terminal output, system logs |

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `F1` | Toggle terminal |
| `F9` | New order (buy) |
| `F10` | New order (sell) |
| `Esc` | Close modal / Cancel operation |
| `Ctrl+K` | Emergency stop |
| `Ctrl+R` | Refresh data |

### SL/TP Management

1. **View Levels**: SL/TP lines auto-appear on chart for open positions
2. **Modify**: Double-click a line to edit price
3. **Drag Mode**: Enable drag mode to visually adjust levels
4. **Confirm**: Changes are sent to backend and broadcast to all clients

## 🔧 Troubleshooting

### Quick Fix Scripts

- **Windows Defender Fix**: Run `scripts/fix-windows-defender.bat` as Administrator
- **Dev Startup**: Use `scripts/start-dev.ps1` to start all services

📚 **See [TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md) for detailed solutions.**

### Windows Defender / Antivirus Issues

If you see `STATUS_ACCESS_VIOLATION` or `Access Denied` during build:

**⭐ Recommended: Use GitHub Codespaces** (build in cloud, no local setup):
- See [CODESPACES_GUIDE.md](CODESPACES_GUIDE.md)

**Local fixes:**
1. **Option 1**: Run `scripts/fix-windows-defender.bat` as Administrator
2. **Option 2**: Manually add exclusion:
   ```powershell
   # Run as Administrator
   Add-MpPreference -ExclusionPath "D:\AI-Projects\ai_pro_v5\desktop-dashboard"
   ```
3. **Option 3**: Temporarily disable real-time protection
4. **Option 4**: Use `cargo clean` and rebuild

### WebSocket Connection Failed

```bash
# Check if port 8081 is available
netstat -ano | findstr :8081

# Kill process using port 8081
taskkill /PID <pid> /F

# Or change port in ws_server.rs and websocket.rs
```

### Frontend Not Loading

```bash
# Regenerate Tailwind CSS
cd ui
npx tailwindcss -i ./styles/main.css -o ./styles/output.css

# Check for compilation errors
cargo check --target wasm32-unknown-unknown
```

## 🧪 Testing

### Paper Trading

1. Set `TINKOFF_TOKEN` in `.env`
2. Run with `--sandbox` flag or set mode in UI
3. All trades execute in sandbox environment
4. Monitor in Tinkoff mobile app

### Mock Data Mode

Without API token, dashboard runs on mock data:
- Simulated price movements
- Generated signals
- Virtual positions

Perfect for UI testing and development.

## 🚧 Roadmap

- [x] Real-time WebSocket streaming
- [x] TradingView chart integration
- [x] Interactive SL/TP drag-and-drop
- [x] Risk management dashboard
- [x] Signal card with animations
- [ ] Tinkoff API integration (in progress)
- [ ] Order book visualization
- [ ] Trade history & analytics
- [ ] Multi-account support
- [ ] Mobile-responsive layout
- [ ] Backtesting module

## 🤝 Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing`)
5. Open Pull Request

## ⚠️ Disclaimer

**Trading financial instruments carries a high level of risk.** This software is provided for educational purposes only:

- Test thoroughly in paper trading mode first
- Never trade with money you cannot afford to lose
- Past performance does not guarantee future results
- The authors are not responsible for any financial losses

## 📜 License

MIT License - see [LICENSE](LICENSE) file

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [ARCHITECTURE.md](docs/ARCHITECTURE.md) | System architecture, data flow, state management |
| [API.md](docs/API.md) | Tauri commands and WebSocket protocol reference |
| [PAPER_TRADING_GUIDE.md](docs/PAPER_TRADING_GUIDE.md) | 7-day paper trading plan with checklist |
| [TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md) | Common issues and solutions |
| [METRICS.md](docs/METRICS.md) | Trading metrics formulas and targets |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Contribution guidelines |

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) - Desktop framework
- [Leptos](https://leptos.dev/) - Rust web framework
- [TradingView](https://www.tradingview.com/) - Charting library
- [Tinkoff Invest API](https://tinkoff.github.io/investAPI/) - Broker integration

---

<p align="center">Built with ❤️ in Rust</p>
