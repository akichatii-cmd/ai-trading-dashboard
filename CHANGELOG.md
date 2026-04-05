# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Added
- WebSocket server with real-time broadcasting
- TradingView Lightweight Charts integration
- Interactive SL/TP drag-and-drop on charts
- Risk management dashboard with animated indicators
- AI signal card with confidence scoring
- Emergency kill switch functionality
- Tauri desktop app framework
- Leptos WASM frontend

### In Progress
- Tinkoff Invest API integration
- Paper trading mode
- Order book visualization
- Trade history tracking

## [0.1.0] - 2024-XX-XX

### Added
- Initial project setup with Tauri 1.5
- Leptos frontend framework integration
- Basic layout: sidebar | chart | sidebar
- WebSocket server on port 8081
- Mock data generator for prices
- TradingView chart with custom datafeed
- SL/TP price line visualization
- Position list component
- Risk metrics display (drawdown, daily loss, position count)
- Signal card with pulse animations
- New order modal
- Collapsible terminal footer
- Dark theme with CSS variables
- Responsive layout for 1920x1080+
- Keyboard shortcuts (F1, F9, F10, Esc, Ctrl+K)

### Technical
- Rust backend with tokio async runtime
- WebSocket client with auto-reconnect
- Tauri commands for trading operations
- Global state management with RwLock
- Tailwind CSS for styling
- TradingView charting library integration

### Security
- Environment variable handling for API tokens
- Input validation on all commands
- Confirmation dialogs for critical actions

---

## Release Notes Template

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Added
- New features

### Changed
- Changes in existing functionality

### Deprecated
- Soon-to-be removed features

### Removed
- Now removed features

### Fixed
- Bug fixes

### Security
- Security improvements
```
