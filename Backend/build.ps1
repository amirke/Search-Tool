param(
    [Parameter()]
    [ValidateSet('dev', 'release')]
    [string]$BuildType = 'dev'
)

# Set environment variables based on build type
if ($BuildType -eq 'release') {
    $env:RUST_BACKTRACE = "1"
    $env:TAURI_PRIVATE_KEY = ""
    $env:TAURI_KEY_PASSWORD = ""
    Write-Host "Building RELEASE version..." -ForegroundColor Green
} else {
    Write-Host "Building DEVELOPMENT version..." -ForegroundColor Yellow
}

# Build the application
try {
    if ($BuildType -eq 'release') {
        # Build release version with MSI installer
        npm run build
        tauri build
    } else {
        # Build and run development version
        npm run build
        tauri dev
    }
} catch {
    Write-Host "Build failed: $_" -ForegroundColor Red
    exit 1
}

# If successful, show where to find the output
if ($BuildType -eq 'release') {
    $outputPath = "src-tauri/target/release/bundle"
    Write-Host "`nBuild completed successfully!" -ForegroundColor Green
    Write-Host "You can find the installer at:" -ForegroundColor Cyan
    Write-Host "$outputPath/msi/Search Tool_1.0.0_x64_en-US.msi" -ForegroundColor Cyan
    Write-Host "`nAnd the standalone EXE at:" -ForegroundColor Cyan
    Write-Host "$outputPath/msi/Search Tool_1.0.0_x64_en-US.exe" -ForegroundColor Cyan
} 