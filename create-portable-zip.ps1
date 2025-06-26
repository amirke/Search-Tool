# SearchTool Portable ZIP Creator
# Run this script after building with: npm run tauri:build

Write-Host "=== SearchTool Portable ZIP Creator ===" -ForegroundColor Green
Write-Host ""

# Step 1: Clean up old portable directory
Write-Host "🧹 Cleaning up old files..." -ForegroundColor Yellow
if (Test-Path "SearchTool-Portable") {
    Remove-Item "SearchTool-Portable" -Recurse -Force
}
if (Test-Path "SearchTool-v*.zip") {
    Remove-Item "SearchTool-v*.zip" -Force
}

# Step 2: Create portable directory
Write-Host "📁 Creating portable directory..." -ForegroundColor Yellow
New-Item -ItemType Directory -Force -Path "SearchTool-Portable" | Out-Null

# Step 3: Copy main executable (note: built as search-tool.exe, renamed to Search Tool.exe)
Write-Host "📋 Copying main executable..." -ForegroundColor Yellow
Copy-Item "Backend/target/release/search-tool.exe" "SearchTool-Portable/Search Tool.exe" -Force

# Step 4: Copy ripgrep engine
Write-Host "📋 Copying ripgrep engine..." -ForegroundColor Yellow
Copy-Item "Backend/bin/rg.exe" "SearchTool-Portable/rg.exe" -Force

# Step 5: Copy WebView2 installer
Write-Host "📋 Copying WebView2 installer..." -ForegroundColor Yellow
Copy-Item "Backend/MicrosoftEdgeWebView2RuntimeInstaller.exe" "SearchTool-Portable/WebView2Installer.exe" -Force

# Step 6: Create README.txt
Write-Host "📝 Creating README.txt..." -ForegroundColor Yellow
@"
=== SEARCH TOOL - PORTABLE VERSION ===

🚀 NO INSTALLATION REQUIRED - RUN DIRECTLY! 🚀

This is a PORTABLE version that bypasses Windows SmartScreen warnings.

📋 FIRST TIME SETUP:
1. If WebView2 is not installed, run: WebView2Installer.exe
2. Double-click: "Search Tool.exe" to start

✅ FEATURES:
- No installation required
- No registry changes
- No administrator privileges needed
- Runs from any folder
- No Windows Defender SmartScreen warnings
- Expand/Collapse toggle button (NEW!)
- Results expanded by default (NEW!)
- Updated UI improvements (NEW!)

🔧 REQUIREMENTS:
- Windows 10/11
- Microsoft Edge WebView2 (included installer: WebView2Installer.exe)

📝 FILES INCLUDED:
- Search Tool.exe (Main application - UPDATED with latest changes)
- rg.exe (Ripgrep search engine)
- WebView2Installer.exe (WebView2 runtime)
- README.txt (This file)
- First-Run.bat (Automated setup)

🆕 LATEST UPDATES:
- Fixed expand/collapse functionality
- Button replaces checkbox for better UX
- Results show expanded by default
- All dev improvements included
- Fresh build with latest source code

🌟 ENJOY SAFE AND FAST TEXT SEARCHING! 🌟
"@ | Out-File -FilePath "SearchTool-Portable/README.txt" -Encoding UTF8

# Step 7: Create First-Run.bat
Write-Host "📝 Creating First-Run.bat..." -ForegroundColor Yellow
@"
@echo off
echo.
echo === SEARCH TOOL - FIRST RUN SETUP ===
echo.
echo Checking for Microsoft Edge WebView2...
echo.

:: Check if WebView2 is installed
reg query "HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" >nul 2>&1
if %errorlevel% equ 0 (
    echo ✅ WebView2 is already installed!
    goto :runapp
)

reg query "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" >nul 2>&1
if %errorlevel% equ 0 (
    echo ✅ WebView2 is already installed!
    goto :runapp
)

echo ⚠️  WebView2 not found. Installing...
echo.
WebView2Installer.exe /silent /install
if %errorlevel% neq 0 (
    echo ❌ WebView2 installation failed. Please run WebView2Installer.exe manually.
    pause
    exit /b 1
)

echo ✅ WebView2 installed successfully!
echo.

:runapp
echo 🚀 Starting Search Tool...
echo.
start "" "Search Tool.exe"
echo.
echo ✅ Search Tool is now running!
echo ✋ You can close this window.
echo.
timeout /t 3 >nul
exit
"@ | Out-File -FilePath "SearchTool-Portable/First-Run.bat" -Encoding ASCII

# Step 8: Get version from tauri.conf.json
Write-Host "🔍 Getting version..." -ForegroundColor Yellow
$tauriConfig = Get-Content "Backend/tauri.conf.json" | ConvertFrom-Json
$version = $tauriConfig.version

# Step 9: Create ZIP
Write-Host "📦 Creating ZIP package..." -ForegroundColor Yellow
$zipName = "SearchTool-v$version-Portable-FINAL.zip"
Compress-Archive -Path "SearchTool-Portable\*" -DestinationPath $zipName -Force

# Step 10: Verify and show results
Write-Host "✅ Verification..." -ForegroundColor Green
$zipInfo = Get-ChildItem $zipName | Select-Object Name, Length
$sizeInMB = [math]::Round($zipInfo.Length / 1MB, 1)

Write-Host ""
Write-Host "🎉 SUCCESS! Portable ZIP created:" -ForegroundColor Green
Write-Host "   File: $($zipInfo.Name)" -ForegroundColor Cyan
Write-Host "   Size: $sizeInMB MB" -ForegroundColor Cyan
Write-Host ""
Write-Host "📁 Contents:" -ForegroundColor Yellow
Get-ChildItem "SearchTool-Portable" | ForEach-Object {
    $fileSize = if ($_.Length) { [math]::Round($_.Length / 1MB, 1) } else { "0" }
    Write-Host "   - $($_.Name) ($fileSize MB)" -ForegroundColor White
}
Write-Host ""
Write-Host "🚀 Ready for distribution!" -ForegroundColor Green 