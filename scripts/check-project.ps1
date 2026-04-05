# Project Health Check Script
# Run: .\scripts\check-project.ps1

param(
    [switch]$Fix
)

$ErrorActionPreference = "Continue"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Project Health Check                 " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

$projectRoot = Split-Path $PSScriptRoot -Parent
Set-Location $projectRoot

$issues = @()
$warnings = @()

# Check 1: Required files exist
Write-Host "Checking project structure..." -ForegroundColor Yellow
$requiredFiles = @(
    "src-tauri\Cargo.toml",
    "src-tauri\src\main.rs",
    "ui\Cargo.toml",
    "ui\index.html",
    "ui\styles\main.css",
    "tailwind.config.js",
    "README.md"
)

foreach ($file in $requiredFiles) {
    if (-not (Test-Path $file)) {
        $issues += "Missing file: $file"
    }
}
Write-Host "  Files check complete" -ForegroundColor Green

# Check 2: Rust installation
Write-Host "Checking Rust toolchain..." -ForegroundColor Yellow
try {
    $rustVersion = rustc --version 2>$null
    if ($rustVersion) {
        Write-Host "  Rust: $rustVersion" -ForegroundColor Green
    } else {
        $issues += "Rust not installed"
    }
    
    # Check WASM target
    $wasmTarget = rustup target list --installed 2>$null | Select-String "wasm32"
    if (-not $wasmTarget) {
        $warnings += "WASM target not installed. Run: rustup target add wasm32-unknown-unknown"
    }
} catch {
    $issues += "Cannot check Rust installation"
}

# Check 3: Node.js and tools
Write-Host "Checking Node.js tools..." -ForegroundColor Yellow
try {
    $nodeVersion = node --version 2>$null
    if ($nodeVersion) {
        Write-Host "  Node.js: $nodeVersion" -ForegroundColor Green
    } else {
        $warnings += "Node.js not installed (optional, for Tailwind)"
    }
} catch {
    $warnings += "Node.js check failed"
}

# Check 4: Tauri CLI
Write-Host "Checking Tauri CLI..." -ForegroundColor Yellow
try {
    $tauriHelp = cargo tauri --help 2>$null
    if ($tauriHelp) {
        Write-Host "  Tauri CLI: installed" -ForegroundColor Green
    } else {
        $warnings += "Tauri CLI not installed. Run: cargo install tauri-cli"
    }
} catch {
    $warnings += "Tauri CLI check failed"
}

# Check 5: Trunk
Write-Host "Checking Trunk..." -ForegroundColor Yellow
try {
    $trunkVersion = trunk --version 2>$null
    if ($trunkVersion) {
        Write-Host "  Trunk: $trunkVersion" -ForegroundColor Green
    } else {
        $warnings += "Trunk not installed. Run: cargo install trunk"
    }
} catch {
    $warnings += "Trunk check failed"
}

# Check 6: Environment file
Write-Host "Checking environment..." -ForegroundColor Yellow
if (Test-Path "src-tauri\.env") {
    Write-Host "  .env file: exists" -ForegroundColor Green
    $envContent = Get-Content "src-tauri\.env" -Raw
    if ($envContent -match "TINKOFF_TOKEN=your_") {
        $warnings += "TINKOFF_TOKEN not configured in .env"
    }
} else {
    $warnings += ".env file not found. Copy .env.example to .env"
}

# Check 7: Windows Defender exclusion
Write-Host "Checking Windows Defender..." -ForegroundColor Yellow
try {
    $exclusions = Get-MpPreference | Select-Object -ExpandProperty ExclusionPath
    if ($exclusions -contains $projectRoot) {
        Write-Host "  Project folder excluded: YES" -ForegroundColor Green
    } else {
        $warnings += "Project folder not in Windows Defender exclusions. Run scripts/fix-windows-defender.bat as Admin"
    }
} catch {
    $warnings += "Cannot check Windows Defender settings (may need admin rights)"
}

# Check 8: Port availability
Write-Host "Checking ports..." -ForegroundColor Yellow
$port8081 = Get-NetTCPConnection -LocalPort 8081 -ErrorAction SilentlyContinue
if ($port8081) {
    $warnings += "Port 8081 is in use. WebSocket server may fail to start."
} else {
    Write-Host "  Port 8081: available" -ForegroundColor Green
}

# Summary
Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Check Summary                        " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

if ($issues.Count -eq 0 -and $warnings.Count -eq 0) {
    Write-Host "✅ All checks passed! Project is ready." -ForegroundColor Green
    Write-Host ""
    Write-Host "Quick start:" -ForegroundColor Yellow
    Write-Host "  1. cd src-tauri; cargo run"
    Write-Host "  2. cd ui; trunk serve"
    Write-Host ""
} else {
    if ($issues.Count -gt 0) {
        Write-Host "❌ Issues found ($($issues.Count)):" -ForegroundColor Red
        foreach ($issue in $issues) {
            Write-Host "   - $issue" -ForegroundColor Red
        }
        Write-Host ""
    }
    
    if ($warnings.Count -gt 0) {
        Write-Host "⚠️  Warnings ($($warnings.Count)):" -ForegroundColor Yellow
        foreach ($warning in $warnings) {
            Write-Host "   - $warning" -ForegroundColor Yellow
        }
        Write-Host ""
    }
    
    if ($Fix -and $warnings.Count -gt 0) {
        Write-Host "Attempting to fix warnings..." -ForegroundColor Cyan
        
        # Try to install missing tools
        if ($warnings -match "WASM target") {
            Write-Host "Installing WASM target..." -ForegroundColor Yellow
            rustup target add wasm32-unknown-unknown
        }
        
        if ($warnings -match "Tauri CLI") {
            Write-Host "Installing Tauri CLI..." -ForegroundColor Yellow
            cargo install tauri-cli
        }
        
        if ($warnings -match "Trunk") {
            Write-Host "Installing Trunk..." -ForegroundColor Yellow
            cargo install trunk
        }
        
        if ($warnings -match ".env file") {
            Write-Host "Creating .env from template..." -ForegroundColor Yellow
            Copy-Item "src-tauri\.env.example" "src-tauri\.env"
        }
        
        Write-Host ""
        Write-Host "Fix attempts complete. Run check again to verify." -ForegroundColor Green
    }
}

# Next steps
Write-Host ""
Write-Host "Useful commands:" -ForegroundColor Cyan
Write-Host "  .\scripts\start-dev.ps1      - Start all services"
Write-Host "  .\scripts\check-project.ps1  - Run health check"
Write-Host "  cargo tauri build            - Build release"
Write-Host ""
