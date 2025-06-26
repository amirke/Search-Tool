# SearchTool Setup Script
# This script helps set up the development environment for the Search Tool project

Write-Host "🚀 Setting up Search Tool Development Environment..." -ForegroundColor Green

# Check prerequisites
Write-Host "📋 Checking prerequisites..." -ForegroundColor Yellow

# Check Node.js
try {
    $nodeVersion = node --version
    Write-Host "✅ Node.js found: $nodeVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Node.js not found. Please install Node.js first." -ForegroundColor Red
    exit 1
}

# Check Rust
try {
    $rustVersion = rustc --version
    Write-Host "✅ Rust found: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Rust not found. Please install Rust first." -ForegroundColor Red
    exit 1
}

# Check Cargo
try {
    $cargoVersion = cargo --version
    Write-Host "✅ Cargo found: $cargoVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Cargo not found. Please install Cargo first." -ForegroundColor Red
    exit 1
}

# Install npm dependencies
Write-Host "📦 Installing npm dependencies..." -ForegroundColor Yellow
npm install
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Failed to install npm dependencies" -ForegroundColor Red
    exit 1
}

# Install Tauri CLI
Write-Host "🔧 Ensuring Tauri CLI is available..." -ForegroundColor Yellow
npm install -g @tauri-apps/cli --force
if ($LASTEXITCODE -ne 0) {
    Write-Host "⚠️ Warning: Failed to install Tauri CLI globally, but it might work with npx" -ForegroundColor Yellow
}

# Check if ripgrep is needed (download if not present)
$rgPath = "Backend\bin\rg.exe"
if (-not (Test-Path $rgPath)) {
    Write-Host "📥 Downloading ripgrep..." -ForegroundColor Yellow
    & "Scripts\download-rg.ps1"
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Failed to download ripgrep" -ForegroundColor Red
        exit 1
    }
}

Write-Host ""
Write-Host "✅ Setup completed successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "🎯 Next steps:" -ForegroundColor Cyan
Write-Host "  • Run 'npm run tauri:dev' to start development mode" -ForegroundColor White
Write-Host "  • Run 'npm run tauri:build' to create a production build" -ForegroundColor White
Write-Host "  • Check README.md for detailed documentation" -ForegroundColor White
Write-Host ""
Write-Host "📁 Project Structure:" -ForegroundColor Cyan
Write-Host "  • Frontend/  - Svelte frontend application" -ForegroundColor White
Write-Host "  • Backend/   - Rust/Tauri backend application" -ForegroundColor White
Write-Host "  • Scripts/   - Build and automation scripts" -ForegroundColor White
Write-Host "  • Documentation/ - Project documentation" -ForegroundColor White
Write-Host ""
Write-Host "Happy coding! 🎉" -ForegroundColor Green 