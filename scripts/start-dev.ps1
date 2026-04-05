# AI Trading Bot Dashboard - Development Startup Script
# Run: .\scripts\start-dev.ps1

param(
    [switch]$BackendOnly,
    [switch]$FrontendOnly,
    [switch]$BuildRelease
)

$ErrorActionPreference = "Stop"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  AI Trading Bot Dashboard - Dev Start  " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

$projectRoot = Split-Path $PSScriptRoot -Parent

function Start-Backend {
    Write-Host "Starting Backend..." -ForegroundColor Yellow
    Write-Host "  Path: $projectRoot\src-tauri" -ForegroundColor Gray
    
    $backendJob = Start-Job -ScriptBlock {
        param($path)
        Set-Location $path
        cargo run 2>&1
    } -ArgumentList "$projectRoot\src-tauri"
    
    Start-Sleep -Seconds 5
    
    # Check if backend started successfully
    $jobOutput = Receive-Job -Job $backendJob -Keep | Select-Object -Last 20
    if ($jobOutput -match "error" -or $jobOutput -match "FAILED") {
        Write-Host "Backend failed to start!" -ForegroundColor Red
        Receive-Job -Job $backendJob
        Stop-Job -Job $backendJob
        exit 1
    }
    
    Write-Host "  Backend started! (WebSocket: ws://127.0.0.1:8081)" -ForegroundColor Green
    return $backendJob
}

function Start-Frontend {
    Write-Host "Starting Frontend..." -ForegroundColor Yellow
    Write-Host "  Path: $projectRoot\ui" -ForegroundColor Gray
    
    $frontendJob = Start-Job -ScriptBlock {
        param($path)
        Set-Location $path
        trunk serve 2>&1
    } -ArgumentList "$projectRoot\ui"
    
    Start-Sleep -Seconds 3
    
    Write-Host "  Frontend started! (http://localhost:8080)" -ForegroundColor Green
    return $frontendJob
}

function Start-Tailwind {
    Write-Host "Starting Tailwind CSS Watch..." -ForegroundColor Yellow
    
    $tailwindJob = Start-Job -ScriptBlock {
        param($path)
        Set-Location $path
        npx tailwindcss -i ./styles/main.css -o ./styles/output.css --watch 2>&1
    } -ArgumentList "$projectRoot\ui"
    
    Write-Host "  Tailwind watch started!" -ForegroundColor Green
    return $tailwindJob
}

function Show-Status {
    Write-Host ""
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host "  Services Status                      " -ForegroundColor Cyan
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host "Backend:  http://localhost:8080 (Tauri)" -ForegroundColor Gray
    Write-Host "WebSocket: ws://localhost:8081" -ForegroundColor Gray
    Write-Host "Frontend: http://localhost:8080" -ForegroundColor Gray
    Write-Host ""
    Write-Host "Press Ctrl+C to stop all services" -ForegroundColor Yellow
    Write-Host ""
}

# Main execution
try {
    if ($BuildRelease) {
        Write-Host "Building Release Version..." -ForegroundColor Yellow
        Set-Location $projectRoot
        cargo tauri build
        exit 0
    }
    
    $jobs = @()
    
    if (-not $FrontendOnly) {
        $jobs += Start-Backend
    }
    
    if (-not $BackendOnly) {
        $jobs += Start-Frontend
        $jobs += Start-Tailwind
    }
    
    Show-Status
    
    # Monitor jobs
    while ($true) {
        foreach ($job in $jobs) {
            $output = Receive-Job -Job $job
            if ($output) {
                Write-Host $output
            }
            
            if ($job.State -eq "Failed") {
                Write-Host "A job failed! Exiting..." -ForegroundColor Red
                exit 1
            }
        }
        Start-Sleep -Milliseconds 100
    }
}
finally {
    Write-Host "`nStopping all services..." -ForegroundColor Yellow
    Get-Job | Stop-Job -ErrorAction SilentlyContinue
    Get-Job | Remove-Job -ErrorAction SilentlyContinue
    Write-Host "Cleanup complete." -ForegroundColor Green
}
