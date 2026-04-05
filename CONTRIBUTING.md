# Contributing to AI Trading Bot Dashboard

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing to the project.

## 🚀 Getting Started

### Development Setup

1. **Fork and clone**
   ```bash
   git clone https://github.com/yourusername/ai-trading-dashboard.git
   cd ai-trading-dashboard
   ```

2. **Install dependencies**
   ```bash
   # Rust
   rustup update
   rustup target add wasm32-unknown-unknown
   
   # Node.js tools
   npm install -g @tauri-apps/cli trunk tailwindcss
   ```

3. **Build and run**
   ```bash
   # Terminal 1: Backend
   cd src-tauri && cargo run
   
   # Terminal 2: Frontend
   cd ui && trunk serve
   ```

## 📋 Code Style

### Rust

- Use `cargo fmt` before committing
- Follow `cargo clippy` suggestions
- Document public APIs with `///`
- Use meaningful variable names

```rust
// Good
pub async fn place_order(
    symbol: String,
    side: OrderSide,
    quantity: Decimal,
) -> Result<Order, TradingError> {
    // Implementation
}

// Avoid
pub fn order(s: String, sd: String, q: f64) -> Result<Order, String> {
    // Implementation
}
```

### CSS/Tailwind

- Use Tailwind utility classes when possible
- Custom CSS only for complex animations
- Follow BEM naming for custom classes
- Maintain dark theme consistency

```css
/* Good */
.btn-primary {
  @apply bg-primary hover:bg-primary-light text-white px-4 py-2 rounded;
}

/* Avoid */
.button {
  background: blue;
  padding: 10px;
}
```

## 🏗️ Project Structure

When adding new features:

```
New feature: Order Book

1. Backend (src-tauri/src/)
   ├── api.rs           - Add tauri command
   ├── ws_server.rs     - Add message type
   └── state.rs         - Add state if needed

2. Frontend (ui/src/)
   ├── components/
   │   └── order_book.rs - New component
   ├── websocket.rs      - Add message handler
   └── app.rs            - Include component

3. Styling (ui/styles/)
   └── Add to main.css or component styles
```

## 🔍 Testing

### Before Submitting PR

```bash
# 1. Format code
cargo fmt --all

# 2. Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# 3. Build frontend
cd ui && trunk build --release

# 4. Build desktop app
cargo tauri build

# 5. Test in both modes:
# - Mock data (no env vars)
# - Paper trading (with TINKOFF_TOKEN)
```

## 📝 Commit Messages

Use conventional commits:

```
feat: add order book visualization
fix: resolve WebSocket reconnection issue
docs: update API documentation
style: format with cargo fmt
refactor: extract common WebSocket logic
test: add integration tests for orders
chore: update dependencies
```

## 🎯 Areas for Contribution

### High Priority

- [ ] Tinkoff API integration
- [ ] Order book visualization (depth chart)
- [ ] Trade history and analytics
- [ ] Performance optimization

### Medium Priority

- [ ] Additional chart indicators
- [ ] Multi-timeframe analysis
- [ ] Keyboard shortcut customization
- [ ] UI theme options

### Documentation

- [ ] API documentation
- [ ] User guide
- [ ] Video tutorials
- [ ] Translation to other languages

## 🐛 Bug Reports

When reporting bugs, include:

1. **Environment**
   - OS: Windows 10/11, macOS, Linux
   - Rust version: `rustc --version`
   - Node version: `node --version`

2. **Steps to reproduce**
3. **Expected behavior**
4. **Actual behavior**
5. **Screenshots** if applicable
6. **Logs** from terminal

## 💡 Feature Requests

Feature requests are welcome! Please:

1. Check existing issues first
2. Describe the use case
3. Explain why it would be useful
4. Consider implementation complexity

## 🔒 Security

Never commit:
- API tokens or credentials
- Private keys
- Personal trading data
- `.env` files

Report security vulnerabilities privately to maintainers.

## 📞 Questions?

- Open an issue for questions
- Join discussions
- Check existing documentation

## 🏅 Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Assume good intentions

Thank you for contributing! 🎉
