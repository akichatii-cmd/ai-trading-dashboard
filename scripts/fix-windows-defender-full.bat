@echo off
chcp 65001 >nul
echo ========================================
echo  Windows Defender Complete Fix
echo ========================================
echo.

:: Check for admin privileges
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo ERROR: Administrator privileges required!
    echo.
    echo Right-click on this file and select "Run as administrator"
    echo.
    pause
    exit /b 1
)

:: Get the project root (parent of scripts folder)
set "SCRIPT_DIR=%~dp0"
set "PROJECT_DIR=%SCRIPT_DIR%.."

:: Get absolute path
for %%I in ("%PROJECT_DIR%") do set "PROJECT_PATH=%%~fI"
set "TARGET_PATH=%PROJECT_PATH%\target"

echo Adding exclusions:
echo 1. Project: %PROJECT_PATH%
echo 2. Target: %TARGET_PATH%
echo.

:: Add project folder
powershell -Command "Add-MpPreference -ExclusionPath '%PROJECT_PATH%'"
if %errorLevel% neq 0 (
    echo Failed to add project folder
    pause
    exit /b 1
)

:: Add target folder (if exists)
if exist "%TARGET_PATH%" (
    powershell -Command "Add-MpPreference -ExclusionPath '%TARGET_PATH%'"
    if %errorLevel% equ 0 (
        echo Target folder added.
    )
)

:: Also add Rust toolchain folders
echo.
echo Adding Rust toolchain folders...
set "CARGO_PATH=C:\Users\%USERNAME%\.cargo"
set "RUSTUP_PATH=C:\Users\%USERNAME%\.rustup"

if exist "%CARGO_PATH%" (
    powershell -Command "Add-MpPreference -ExclusionPath '%CARGO_PATH%'" 2>nul
    echo  - .cargo added
)

if exist "%RUSTUP_PATH%" (
    powershell -Command "Add-MpPreference -ExclusionPath '%RUSTUP_PATH%'" 2>nul
    echo  - .rustup added
)

echo.
echo ========================================
echo  SUCCESS! All exclusions added.
echo ========================================
echo.
echo IMPORTANT: Close ALL terminals and IDEs now!
echo Then run:
echo   cargo clean
echo   cargo build
echo.
pause
