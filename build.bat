@echo off
chcp 65001 >nul
echo ==========================================
echo AI Trading Bot Dashboard - Build Script
echo ==========================================
echo.

:: Check Rust
echo [1/4] Checking Rust installation...
rustc --version >nul 2>&1
if errorlevel 1 (
    echo ERROR: Rust not found. Please install from https://rustup.rs/
    exit /b 1
)
echo ✓ Rust found

:: Check Node.js
echo [2/4] Checking Node.js...
node --version >nul 2>&1
if errorlevel 1 (
    echo WARNING: Node.js not found. Some features may not work.
) else (
    echo ✓ Node.js found
)

:: Build UI (if trunk is installed)
echo [3/4] Building Web UI...
trunk --version >nul 2>&1
if errorlevel 1 (
    echo WARNING: Trunk not found. Skipping UI build.
    echo Install with: cargo install trunk
) else (
    cd ui
    echo Building UI with trunk...
    trunk build --release
    cd ..
)

:: Build Tauri
echo [4/4] Building Tauri Application...
cd src-tauri

echo.
echo Select build mode:
echo [1] Development (cargo run)
echo [2] Production (cargo tauri build)
echo.
set /p choice="Enter choice (1 or 2): "

if "%choice%"=="1" (
    echo Starting development server...
    cargo tauri dev
) else if "%choice%"=="2" (
    echo Building production release...
    cargo tauri build
    echo.
    echo Build complete! Check src-tauri/target/release/bundle/
) else (
    echo Invalid choice
    exit /b 1
)

cd ..
echo.
echo ==========================================
echo Build process completed!
echo ==========================================
pause
