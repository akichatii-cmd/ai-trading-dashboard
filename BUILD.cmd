@echo off
chcp 65001 >nul
echo ========================================
echo  Building AI Trading Bot Dashboard
echo ========================================
echo.

:: Check admin
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo ERROR: Run as Administrator!
    pause
    exit /b 1
)

:: Show toolchain
echo Rust toolchain:
rustup show active-toolchain
echo.

:: Disable Defender
echo [1/3] Disabling Windows Defender...
powershell -Command "Set-MpPreference -DisableRealtimeMonitoring $true"
echo      Done.
echo.

:: Clean
echo [2/3] Cleaning...
cd /d "D:\AI-Projects\ai_pro_v5\desktop-dashboard"
cargo clean 2>nul
echo      Done.
echo.

:: Build
echo [3/3] Building release (this takes 10-15 min)...
cd src-tauri
cargo build --release
set BUILD_RESULT=%errorLevel%

:: Re-enable Defender
echo.
echo Re-enabling Windows Defender...
powershell -Command "Set-MpPreference -DisableRealtimeMonitoring $false"
echo Done.

:: Check result
if %BUILD_RESULT% == 0 (
    echo.
    echo ========================================
    echo  BUILD SUCCESS!
    echo ========================================
    echo.
    echo Run with:
    echo   .\src-tauri\target\release\desktop-dashboard-tauri.exe
) else (
    echo.
    echo ========================================
    echo  BUILD FAILED
    echo ========================================
)

echo.
pause
