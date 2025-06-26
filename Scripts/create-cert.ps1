# Create a self-signed certificate
$cert = New-SelfSignedCertificate -Type Custom -Subject "CN=Search Tool" -KeyUsage DigitalSignature -FriendlyName "Search Tool Certificate" -CertStoreLocation "Cert:\CurrentUser\My" -TextExtension @("2.5.29.37={text}1.3.6.1.5.5.7.3.3", "2.5.29.19={text}")

# Export the certificate
$certPath = "Cert:\CurrentUser\My\$($cert.Thumbprint)"
Export-Certificate -Cert $certPath -FilePath "SearchTool.cer"

# Get the certificate thumbprint
$thumbprint = $cert.Thumbprint

# Update tauri.conf.json with the certificate thumbprint
$tauriConfig = Get-Content "src-tauri/tauri.conf.json" | ConvertFrom-Json
$tauriConfig.tauri.bundle.windows.certificateThumbprint = $thumbprint
$tauriConfig | ConvertTo-Json -Depth 10 | Set-Content "src-tauri/tauri.conf.json"

Write-Host "Certificate created and configuration updated. Thumbprint: $thumbprint" 