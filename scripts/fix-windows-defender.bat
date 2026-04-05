@echo off
chcp 65001 >nul
echo ========================================
echo  Windows Defender Fix for Rust/Tauri
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

echo Adding project folder to exclusions...
echo Path: %PROJECT_PATH%
echo.

powershell -Command "Add-MpPreference -ExclusionPath '%PROJECT_PATH%'"

if %errorLevel% equ 0 (
    echo Success! Project folder added to exclusions.
) else (
    echo Failed to add exclusion. Error code: %errorLevel%
    pause
    exit /b 1
)

echo.
echo Current exclusions:
powershell -Command "Get-MpPreference | Select-Object -Property ExclusionPath | Format-List"

echo.
echo ========================================
echo  Next steps:
echo ========================================
echo 1. Close all terminals/IDEs
echo 2. Run: cargo clean
echo 3. Run: cargo build
echo.
echo The STATUS_ACCESS_VIOLATION error should be fixed!
echo.
pause
