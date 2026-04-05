@echo off
chcp 65001 >nul
setlocal EnableDelayedExpansion

echo ========================================
echo  Build with Defender Temporarily Off
echo ========================================
echo.

:: Check for admin privileges
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo ERROR: Administrator privileges required!
    pause
    exit /b 1
)

:: Get project path
set "SCRIPT_DIR=%~dp0"
set "PROJECT_DIR=%SCRIPT_DIR%.."
for %%I in ("%PROJECT_DIR%") do set "PROJECT_PATH=%%~fI"

echo [1/5] Stopping Windows Defender real-time protection...
powershell -Command "Set-MpPreference -DisableRealtimeMonitoring $true" 2>nul
if %errorLevel% neq 0 (
    echo      WARNING: Could not disable Defender. Will try anyway...
) else (
    echo      Done. Defender temporarily disabled.
)
echo.

echo [2/5] Cleaning build cache...
cd /d "%PROJECT_PATH%"
cargo clean 2>nul
echo      Done.
echo.

echo [3/5] Building backend (this may take 10-15 minutes)...
cd /d "%PROJECT_PATH%\src-tauri"
cargo build --release
set BUILD_STATUS=%errorLevel%

echo.
echo [4/5] Checking build result...
if exist "%PROJECT_PATH%\src-tauri\target\release\desktop-dashboard-tauri.exe" (
    echo      SUCCESS! Executable created.
    set BUILD_SUCCESS=1
) else (
    echo      FAILED! Executable not found.
    set BUILD_SUCCESS=0
)
echo.

echo [5/5] Restarting Windows Defender...
powershell -Command "Set-MpPreference -DisableRealtimeMonitoring $false" 2>nul
echo      Done.
echo.

if "%BUILD_SUCCESS%"=="1" (
    echo ========================================
    echo  BUILD SUCCESS!                        
    echo ========================================
    echo.
    echo Executable:
    echo   %PROJECT_PATH%\src-tauri\target\release\desktop-dashboard-tauri.exe
    echo.
    echo Run with:
    echo   .\src-tauri\target\release\desktop-dashboard-tauri.exe
) else (
    echo ========================================
    echo  BUILD FAILED                          
    echo ========================================
    echo.
    echo Possible solutions:
    echo 1. Run Windows Security ^> Virus protection ^> Manage settings
    echo 2. Temporarily turn OFF Real-time protection manually
    echo 3. Then run this script again
    echo.
    echo Alternative: Use WSL2 or GitHub Codespaces
)

echo.
pause
