$url = "https://aka.ms/windowssdk/wsdkdownload"
$output = "winsdk.exe"
Invoke-WebRequest -Uri $url -OutFile $output
Write-Host "Downloaded Windows SDK installer to $output" 