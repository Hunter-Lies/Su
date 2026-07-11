# Su! Sparse Package - Register (Run as Administrator)
$ErrorActionPreference = "Stop"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$manifest = Join-Path $scriptDir "AppxManifest.xml"

if (-not (Test-Path $manifest)) { Write-Host "ERROR: AppxManifest.xml not found" -Foreground Red; exit 1 }

Write-Host "[Su!] Creating self-signed certificate..." -Foreground Cyan
$cert = New-SelfSignedCertificate -Type Custom -Subject "CN=Su" -KeyUsage DigitalSignature -FriendlyName "Su! Sparse Package" -CertStoreLocation "Cert:\CurrentUser\My" -TextExtension @("2.5.29.37={text}1.3.6.1.5.5.7.3.3")

Write-Host "[Su!] Trusting certificate..." -Foreground Cyan
$certPath = Join-Path $env:TEMP "su_sparse.cer"
Export-Certificate -Cert $cert -FilePath $certPath -Type CERT | Out-Null
Import-Certificate -FilePath $certPath -CertStoreLocation "Cert:\CurrentUser\Root" | Out-Null

Write-Host "[Su!] Registering sparse package..." -Foreground Cyan
Add-AppxPackage -Register $manifest -ForceApplicationShutdown

Write-Host "[Su!] Done! Right-click should show '通过 Su! 分享' in primary menu." -Foreground Green
Write-Host "Run unregister_sparse.ps1 to undo." -Foreground Yellow
