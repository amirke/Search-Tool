# Function to clean up build artifacts
function Remove-BuildArtifacts {
    # Stop any running instances
    Stop-Process -Name "Search-Tool" -Force -ErrorAction SilentlyContinue
    
    # Remove old MSI files
    Remove-Item -Path "target\release\bundle\msi\*.msi" -Force -ErrorAction SilentlyContinue
    
    # Clean up old build artifacts
    Remove-Item -Path "target\release\*.exe" -Force -ErrorAction SilentlyContinue
    Remove-Item -Path "target\release\*.pdb" -Force -ErrorAction SilentlyContinue
    Remove-Item -Path "target\release\bundle\temp" -Recurse -Force -ErrorAction SilentlyContinue
}

# Function to wait for a file to be unlocked
function Wait-ForFileUnlock {
    param (
        [string]$FilePath
    )
    
    $locked = $true
    while ($locked) {
        try {
            $file = [System.IO.File]::Open($FilePath, 'Open', 'Read', 'None')
            $file.Close()
            $locked = $false
        }
        catch {
            Write-Host "File is locked, waiting..."
            Start-Sleep -Seconds 1
        }
    }
}

# Function to attempt a single build
function Start-Build {
    try {
        # Run the build command
        npm run tauri build
        
        # Check if MSI was created
        $msiPath = "target\release\bundle\msi\Search-Tool_0.1.0_x64_en-US.msi"
        if (Test-Path $msiPath) {
            Write-Host "Build successful! MSI created at: $msiPath"
            return $true
        }
        else {
            Write-Host "Build completed but MSI file not found at: $msiPath"
            return $false
        }
    }
    catch {
        Write-Host "Build failed with error: $_"
        return $false
    }
}

# Main build process
Write-Host "Starting build process..."

# Clean up first
Remove-BuildArtifacts

# Try to build
$success = Start-Build
if (-not $success) {
    Write-Host "Initial build failed, attempting manual MSI creation..."
    
    # Create necessary directories
    New-Item -ItemType Directory -Force -Path "target\release\bundle\temp"
    
    # Copy executable to temp location
    Copy-Item "target\release\Search-Tool.exe" -Destination "target\release\bundle\temp\Search-Tool.exe"
    
    # Create WiX XML file
    $wixContent = @"
<?xml version="1.0" encoding="UTF-8"?>
<Wix xmlns="http://schemas.microsoft.com/wix/2006/wi">
    <Product Id="*" 
             Name="Search Tool" 
             Language="1033" 
             Version="0.1.0" 
             Manufacturer="Your Company" 
             UpgradeCode="PUT-GUID-HERE">
        <Package InstallerVersion="200" 
                 Compressed="yes" 
                 InstallScope="perMachine" />
        <MediaTemplate EmbedCab="yes" />
        <Feature Id="ProductFeature" Title="Search Tool" Level="1">
            <ComponentGroupRef Id="ProductComponents" />
        </Feature>
        <Directory Id="TARGETDIR" Name="SourceDir">
            <Directory Id="ProgramFiles64Folder">
                <Directory Id="INSTALLFOLDER" Name="Search Tool">
                    <Component Id="ProductComponents" Guid="*">
                        <File Id="ApplicationExecutable" 
                              Source="target\release\bundle\temp\Search-Tool.exe" 
                              KeyPath="yes">
                            <Permission User="Everyone" GenericAll="yes" />
                        </File>
                    </Component>
                </Directory>
            </Directory>
        </Directory>
        <ComponentGroup Id="ProductComponents" Directory="INSTALLFOLDER" />
    </Product>
</Wix>
"@
    
    Set-Content -Path "target\release\bundle\temp\main.wxs" -Value $wixContent
    
    # Run WiX tools
    $candle = "C:\Program Files (x86)\WiX Toolset v3.11\bin\candle.exe"
    $light = "C:\Program Files (x86)\WiX Toolset v3.11\bin\light.exe"
    
    if (Test-Path $candle) {
        & $candle "target\release\bundle\temp\main.wxs" -out "target\release\bundle\temp\"
        if ($LASTEXITCODE -eq 0) {
            & $light "target\release\bundle\temp\main.wixobj" -out "target\release\bundle\msi\Search-Tool_0.1.0_x64_en-US.msi"
            if ($LASTEXITCODE -eq 0) {
                Write-Host "Manual MSI creation successful!"
                $success = $true
            }
        }
    }
}

if ($success) {
    Write-Host "Build process completed successfully!"
    exit 0
} else {
    Write-Host "Build process failed!"
    exit 1
} 