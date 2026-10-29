[CmdletBinding(PositionalBinding = $false)]
param(
    [string]$OutDir = "dist\Vortex-Portable",
    [switch]$NoZip
)

$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot

$Running = Get-Process -Name "vortex", "Vortex" -ErrorAction SilentlyContinue
if ($Running) {
    Write-Host ("[Vortex] Stopping running instance(s) (PID={0})..." -f (($Running | Select-Object -ExpandProperty Id) -join ",")) -ForegroundColor Yellow
    $Running | Stop-Process -Force -ErrorAction SilentlyContinue
    Start-Sleep -Milliseconds 300
}

$sw = [System.Diagnostics.Stopwatch]::StartNew()
Write-Host "[Vortex] Building portable release binary (LTO + stripped)..." -ForegroundColor Cyan
cargo build --release -p vortex-cli --bin vortex
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}
$sw.Stop()

$ReleaseBin = Join-Path $PSScriptRoot "target\release\vortex.exe"
if (-not (Test-Path $ReleaseBin)) {
    throw "Release binary not found at $ReleaseBin"
}

$DestDir = Join-Path $PSScriptRoot $OutDir
$DataDir = Join-Path $DestDir "data"
New-Item -ItemType Directory -Path $DestDir -Force | Out-Null
New-Item -ItemType Directory -Path $DataDir -Force | Out-Null

$PortableExe = Join-Path $DestDir "Vortex.exe"
Copy-Item -Path $ReleaseBin -Destination $PortableExe -Force

$PortableConfig = Join-Path $DestDir "vortex.toml"
if (-not (Test-Path $PortableConfig)) {
    @"
profile = "auto"
database_path = "data/vortex.db"
encrypt_database = false
bind_addr = "127.0.0.1:7788"
clipboard_auto_copy = false
clipboard_clear_secs = 45
post_expiry_action = "mark_read"
"@ | Set-Content -Path $PortableConfig -Encoding UTF8
}

$SizeMb = (Get-Item $PortableExe).Length / 1MB
Write-Host ("[Vortex] Portable folder ready in {0:N2}s -> {1} ({2:N2} MB)" -f $sw.Elapsed.TotalSeconds, $DestDir, $SizeMb) -ForegroundColor Green

if (-not $NoZip) {
    $ZipPath = "$DestDir.zip"
    if (Test-Path $ZipPath) {
        Remove-Item -Path $ZipPath -Force
    }
    Compress-Archive -Path "$DestDir\*" -DestinationPath $ZipPath -CompressionLevel Optimal -Force
    $ZipMb = (Get-Item $ZipPath).Length / 1MB
    Write-Host ("[Vortex] Portable archive ready -> {0} ({1:N2} MB)" -f $ZipPath, $ZipMb) -ForegroundColor Cyan
}

