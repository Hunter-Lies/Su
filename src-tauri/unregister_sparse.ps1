# Su! Sparse Package - Unregister (Run as Administrator)
$ErrorActionPreference = "Continue"
Get-AppxPackage -Name "Su.SuShare" | Remove-AppxPackage -ErrorAction SilentlyContinue
Get-ChildItem "Cert:\CurrentUser\My" | Where-Object { $_.Subject -eq "CN=Su" } | Remove-Item -Force -ErrorAction SilentlyContinue
Get-ChildItem "Cert:\CurrentUser\Root" | Where-Object { $_.Subject -eq "CN=Su" } | Remove-Item -Force -ErrorAction SilentlyContinue
Write-Host "[Su!] Unregistered. Clean." -Foreground Green
