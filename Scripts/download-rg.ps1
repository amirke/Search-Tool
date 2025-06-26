# Create bin directory if it doesn't exist
New-Item -ItemType Directory -Force -Path "src-tauri/bin"

# Download ripgrep
$url = "https://github.com/BurntSushi/ripgrep/releases/download/14.1.0/ripgrep-14.1.0-x86_64-pc-windows-msvc.zip"
$output = "ripgrep.zip"
Invoke-WebRequest -Uri $url -OutFile $output

# Extract ripgrep
Expand-Archive -Path $output -DestinationPath "temp" -Force

# Copy rg.exe to bin directory
Copy-Item "temp/ripgrep-14.1.0-x86_64-pc-windows-msvc/rg.exe" -Destination "src-tauri/bin/rg.exe"

# Clean up
Remove-Item -Path $output
Remove-Item -Path "temp" -Recurse -Force

Write-Host "Ripgrep has been downloaded and placed in src-tauri/bin/" 