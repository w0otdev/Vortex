param(
    [string]$BinaryPath = "target/release/vortex.exe"
)

$ErrorActionPreference = "Stop"

if (-not (Test-Path $BinaryPath)) {
    throw "Release binary not found at $BinaryPath"
}

$binBytes = (Get-Item $BinaryPath).Length
$proc = Start-Process -FilePath $BinaryPath -ArgumentList "--hold-ms", "600" -PassThru -NoNewWindow
Start-Sleep -Milliseconds 150
$proc.Refresh()
$rssBytes = $proc.WorkingSet64
$proc.WaitForExit()

foreach ($tier in @("lite", "standard", "full")) {
    & $BinaryPath --check-budgets $tier --binary-bytes $binBytes --rss-bytes $rssBytes
    if ($LASTEXITCODE -ne 0) {
        throw "Budget check failed for tier $tier"
    }
}
