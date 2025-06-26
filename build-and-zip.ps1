# Build and Create Portable ZIP - One Command
Write-Host "🚀 Building and creating portable ZIP..." -ForegroundColor Green

# Build the application
npm run tauri:build

# Run the portable ZIP creator
& .\create-portable-zip.ps1 