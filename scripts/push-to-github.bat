@echo off
chcp 65001 >nul
echo ========================================
echo  Push to GitHub for Codespaces
echo ========================================
echo.

:: Get project path
set "SCRIPT_DIR=%~dp0"
set "PROJECT_DIR=%SCRIPT_DIR%.."
for %%I in ("%PROJECT_DIR%") do set "PROJECT_PATH=%%~fI"

cd /d "%PROJECT_PATH%"

echo [1/5] Checking Git installation...
git --version >nul 2>&1
if errorlevel 1 (
    echo ERROR: Git not installed!
    echo Download: https://git-scm.com/download/win
    pause
    exit /b 1
)
echo      OK
echo.

echo [2/5] Initializing Git repository...
if not exist ".git" (
    git init
    echo      Repository created
) else (
    echo      Repository already exists
)
echo.

echo [3/5] Adding files to Git...
git add .
echo      Done
echo.

echo [4/5] Creating commit...
git commit -m "Initial commit: AI Trading Dashboard v1.0" 2>nul
if errorlevel 1 (
    echo      No changes to commit (or already committed)
) else (
    echo      Commit created
)
echo.

echo [5/5] Checking remote...
git remote -v >nul 2>&1
if errorlevel 1 (
    echo.
    echo ========================================
    echo  MANUAL STEP REQUIRED
echo ========================================
    echo.
    echo 1. Create repository on GitHub:
    echo    https://github.com/new
    echo.
    echo 2. Name it: ai-trading-dashboard
    echo.
    echo 3. Then run these commands:
    echo    git remote add origin https://github.com/YOUR_USERNAME/ai-trading-dashboard.git
    echo    git push -u origin main
    echo.
    echo Or copy the commands from GitHub after creating repo.
) else (
    echo      Pushing to GitHub...
    git push -u origin main
    if errorlevel 1 (
        echo      Push failed. Check your credentials.
    ) else (
        echo      SUCCESS! Code pushed to GitHub.
    )
)

echo.
echo ========================================
echo  Next Steps:
echo ========================================
echo 1. Go to: https://github.com/YOUR_USERNAME/ai-trading-dashboard
    echo 2. Click: "Code" -^> "Codespaces" -^> "Create codespace"
echo 3. Wait 2-3 minutes for setup
echo 4. Run: cd src-tauri ^&^& cargo build --release
echo.
pause
