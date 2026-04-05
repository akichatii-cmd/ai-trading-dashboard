# Troubleshooting Guide

## Windows Defender / Antivirus Issues

### Problem: `STATUS_ACCESS_VIOLATION` during build

```
error: process didn't exit successfully: `rustc.exe` (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)
```

Or:
```
Отказано в доступе (os error 5)
```

### Solution 1: Run Fix Script (Recommended)

1. Right-click on `scripts/fix-windows-defender.bat`
2. Select **"Run as administrator"**
3. Follow the prompts
4. Run `cargo clean` and rebuild

### Solution 2: PowerShell Command

Open PowerShell as Administrator:

```powershell
# Add project folder to exclusions
Add-MpPreference -ExclusionPath "D:\AI-Projects\ai_pro_v5\desktop-dashboard"

# Verify
Get-MpPreference | Select-Object -Property ExclusionPath
```

### Solution 3: Windows Security GUI

1. Open **Windows Security** → **Virus & threat protection**
2. Click **Manage settings** under "Virus & threat protection settings"
3. Scroll to **Exclusions** → Click **Add or remove exclusions**
4. Click **Add an exclusion** → **Folder**
5. Select: `D:\AI-Projects\ai_pro_v5\desktop-dashboard`

### Solution 4: Temporary Disable (Quick Test)

```powershell
# Disable real-time protection (temporary)
Set-MpPreference -DisableRealtimeMonitoring $true

# Run your build
cargo clean
cargo build

# Re-enable
Set-MpPreference -DisableRealtimeMonitoring $false
```

---

## WebSocket Connection Issues

### Problem: "WebSocket connection failed"

### Check if port 8081 is in use

```powershell
# Windows
netstat -ano | findstr :8081

# Kill process using the port
taskkill /PID <PID> /F
```

### Check firewall

```powershell
# Add firewall rule for WebSocket
New-NetFirewallRule -DisplayName "Trading Dashboard WS" -Direction Inbound -Protocol TCP -LocalPort 8081 -Action Allow
```

---

## Build Issues

### Problem: Cargo.lock conflicts

```bash
cargo clean
rm Cargo.lock  # or del Cargo.lock on Windows
cargo build
```

### Problem: Outdated dependencies

```bash
# Update all dependencies
cargo update

# Check for outdated packages
cargo outdated  # requires: cargo install cargo-outdated
```

### Problem: Missing WASM target

```bash
rustup target add wasm32-unknown-unknown
```

---

## Frontend Issues

### Problem: Tailwind CSS not generated

```bash
cd ui
npx tailwindcss -i ./styles/main.css -o ./styles/output.css
```

Or with watch mode:
```bash
npx tailwindcss -i ./styles/main.css -o ./styles/output.css --watch
```

### Problem: Trunk not found

```bash
cargo install trunk wasm-bindgen-cli
```

### Problem: Blank page in browser

1. Check browser console for errors
2. Verify `output.css` exists in `ui/styles/`
3. Check that trunk is serving: `trunk serve --address 127.0.0.1`

---

## Runtime Issues

### Problem: "Failed to connect to WebSocket"

Check:
1. Is backend running? (`cargo run` in src-tauri)
2. Is port 8081 available?
3. Windows Firewall blocking?

### Problem: Charts not loading

1. Check browser console for JavaScript errors
2. Verify `tradingview.js` is in `ui/public/`
3. Check that chart container has dimensions (width/height)

### Problem: API commands not working

1. Check Tauri permissions in `src-tauri/tauri.conf.json`
2. Verify command is registered in `main.rs`
3. Check for errors in terminal output

---

## Tinkoff API Issues

### Problem: "API token not configured"

1. Copy `src-tauri/.env.example` to `src-tauri/.env`
2. Add your sandbox token from [Tinkoff Invest](https://www.tinkoff.ru/invest/open-api/)
3. Restart the application

### Problem: "Invalid token"

- Ensure you're using **sandbox token**, not production
- Check token hasn't expired
- Verify no extra spaces in `.env` file

### Problem: "Account not found"

- Create sandbox account via Tinkoff API or mobile app
- Set `TINKOFF_ACCOUNT_ID` in `.env`

---

## Performance Issues

### Problem: High CPU usage

```rust
// In ws_server.rs, reduce update frequency
const UPDATE_INTERVAL_MS: u64 = 500;  // Instead of 100
```

### Problem: Memory leaks

Check for:
- Unclosed WebSocket connections
- Forgotten event listeners in JavaScript
- Growing vectors without cleanup

### Problem: UI lag

1. Reduce chart update frequency
2. Use `requestAnimationFrame` for animations
3. Debounce rapid state updates

---

## Getting Help

If issues persist:

1. Check [GitHub Issues](https://github.com/yourusername/ai-trading-dashboard/issues)
2. Run with debug logging: `RUST_LOG=debug cargo run`
3. Collect logs from:
   - Terminal output
   - Browser console (F12)
   - Windows Event Viewer

4. Create a minimal reproduction case
