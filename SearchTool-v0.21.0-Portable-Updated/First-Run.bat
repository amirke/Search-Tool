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