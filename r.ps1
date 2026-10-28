[CmdletBinding(PositionalBinding = $false)]
param(
    [string]$Bind = "127.0.0.1:7788",
    [string]$Config = "vortex.toml",
    [switch]$Headless,
    [switch]$Rebuild,
    [switch]$Release,
    [switch]$SingleInstance = $true,
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$CommandArgs
)

$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot

$IsSubcommand = ($CommandArgs -and $CommandArgs.Count -gt 0)

# Guard against concurrent r.ps1 invocations racing each other
$LauncherMutex = New-Object System.Threading.Mutex($false, "Local\VortexInboxRunnerLock")
$HasMutex = $false
try {
    $HasMutex = $LauncherMutex.WaitOne(5000)
} catch [System.Threading.AbandonedMutexException] {
    $HasMutex = $true
}

try {
    # Always kill any old vortex instance before building or launching the app
    if (-not $IsSubcommand) {
        $OldProcs = Get-Process -Name "vortex" -ErrorAction SilentlyContinue
        if ($OldProcs) {
            Write-Host ("[Vortex] Stopping previous instance (PID={0})..." -f (($OldProcs | Select-Object -ExpandProperty Id) -join ",")) -ForegroundColor Yellow
            $OldProcs | Stop-Process -Force -ErrorAction SilentlyContinue
            for ($w = 0; $w -lt 25; $w++) {
                if (-not (Get-Process -Name "vortex" -ErrorAction SilentlyContinue)) { break }
                Start-Sleep -Milliseconds 25
            }
        }
    }

    # Clean up orphaned per-PID WebView2 temp dirs, session stats, and stray root files
    Get-ChildItem -Path $env:TEMP -Directory -Filter "vortex-inbox-webview2-*" -ErrorAction SilentlyContinue |
        Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
    Remove-Item -Path (Join-Path $PSScriptRoot ".github\context-mode") -Recurse -Force -ErrorAction SilentlyContinue
    Get-ChildItem -Path $PSScriptRoot -File -Include "*.pdb", "*.tmp", "*.log", "vortex.db", "vortex.db-shm", "vortex.db-wal" -ErrorAction SilentlyContinue |
        Remove-Item -Force -ErrorAction SilentlyContinue

    $env:CARGO_INCREMENTAL = "1"
    $ProfileSubdir = if ($Release) { "release" } else { "debug" }
    $BinPath = Join-Path $PSScriptRoot "target\$ProfileSubdir\vortex.exe"

    $NeedsBuild = $Rebuild -or (-not (Test-Path $BinPath))
    if (-not $NeedsBuild) {
        $BinTime = (Get-Item $BinPath).LastWriteTimeUtc
        $LatestSrc = Get-ChildItem -Path (Join-Path $PSScriptRoot "crates") -Recurse -File -Filter "*.rs" |
            Where-Object { $_.FullName -notlike "*\crates\ui-web\src\lib.rs" } |
            Sort-Object LastWriteTimeUtc -Descending |
            Select-Object -First 1
        $RootCargoTime = (Get-Item (Join-Path $PSScriptRoot "Cargo.toml")).LastWriteTimeUtc
        if (($LatestSrc -and $LatestSrc.LastWriteTimeUtc -gt $BinTime) -or ($RootCargoTime -gt $BinTime)) {
            $NeedsBuild = $true
        }
    }

    if ($NeedsBuild) {
        $sw = [System.Diagnostics.Stopwatch]::StartNew()
        Write-Host "[Vortex] Building app (incremental)..." -ForegroundColor Cyan
        if ($Release) {
            cargo build --release -p vortex-cli --bin vortex --quiet
        } else {
            cargo build -p vortex-cli --bin vortex --quiet
        }
        if ($LASTEXITCODE -ne 0) {
            exit $LASTEXITCODE
        }
        $sw.Stop()
        Write-Host ("[Vortex] Build ready in {0:N2}s" -f $sw.Elapsed.TotalSeconds) -ForegroundColor DarkCyan
    }

    if ($IsSubcommand) {
        & $BinPath @CommandArgs --config $Config
        exit $LASTEXITCODE
    }

    $DaemonArgs = @("daemon", "--bind", $Bind, "--config", $Config)
    if ($SingleInstance) {
        $DaemonArgs += "--single-instance"
    } else {
        $DaemonArgs += "--allow-multi"
    }
    if ($Headless) {
        $DaemonArgs += "--headless"
    }
    Write-Host "[Vortex] Launching Vortex Inbox (single-instance)..." -ForegroundColor Green
    $proc = Start-Process -FilePath $BinPath -ArgumentList $DaemonArgs -WorkingDirectory $PSScriptRoot -PassThru

    if (-not $Headless) {
        Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public static class VortexWin32 {
    [DllImport("user32.dll")] public static extern bool AllowSetForegroundWindow(int dwProcessId);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] public static extern IntPtr FindWindow(string lpClassName, string lpWindowName);
    [DllImport("user32.dll")] public static extern bool ShowWindowAsync(IntPtr hWnd, int nCmdShow);
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr hWnd);
    [DllImport("user32.dll")] public static extern bool BringWindowToTop(IntPtr hWnd);
}
"@ -ErrorAction SilentlyContinue

        [VortexWin32]::AllowSetForegroundWindow($proc.Id) | Out-Null
        for ($i = 0; $i -lt 60; $i++) {
            Start-Sleep -Milliseconds 40
            $p = Get-Process -Id $proc.Id -ErrorAction SilentlyContinue
            if (-not $p) { break }
            $hwnd = [VortexWin32]::FindWindow($null, "Vortex Inbox")
            if ($hwnd -eq [IntPtr]::Zero) {
                $hwnd = $p.MainWindowHandle
            }
            if ($hwnd -ne [IntPtr]::Zero) {
                [VortexWin32]::ShowWindowAsync($hwnd, 9) | Out-Null
                [VortexWin32]::BringWindowToTop($hwnd) | Out-Null
                [VortexWin32]::SetForegroundWindow($hwnd) | Out-Null
                Write-Host ("[Vortex] Ready (PID={0}, HWND=0x{1:X})" -f $p.Id, $hwnd.ToInt64()) -ForegroundColor Green
                break
            }
        }
    }
} finally {
    if ($HasMutex) {
        $LauncherMutex.ReleaseMutex()
    }
    $LauncherMutex.Dispose()
}

