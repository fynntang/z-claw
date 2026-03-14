# Ensure zeroclaw binary exists in src-tauri/binaries/ for Tauri sidecar.
# Prefer downloading from GitHub Release; fallback to local build when ZEROCLAW_BUILD_FROM_SOURCE=1 or no asset.
# Usage: run from repo root. Requires: PowerShell 5.1+, optional: git, cargo (for source build).

$ErrorActionPreference = "Stop"
$RepoRoot = $PSScriptRoot + "\.."
$BinariesDir = Join-Path $RepoRoot "src-tauri\binaries"
$ZeroclawRepo = "https://github.com/zeroclaw-labs/zeroclaw"
$ReleaseTag = "v0.2.0"

# Target triple: use env from Tauri/Cargo or detect
$Target = $env:CARGO_BUILD_TARGET
if (-not $Target) {
    $rustcOut = & rustc -vV 2>$null | Select-String "host:"
    if ($rustcOut) { $Target = ($rustcOut -split "host:\s*")[1].Trim() }
}
if (-not $Target) { $Target = "x86_64-pc-windows-msvc" }

$ExeExt = ""
if ($Target -match "windows") { $ExeExt = ".exe" }
$OutName = "zeroclaw-$Target$ExeExt"
$OutPath = Join-Path $BinariesDir $OutName

New-Item -ItemType Directory -Force -Path $BinariesDir | Out-Null

if (Test-Path $OutPath) {
    Write-Host "zeroclaw binary exists: $OutPath"
    exit 0
}

$BuildFromSource = $env:ZEROCLAW_BUILD_FROM_SOURCE -eq "1"

function Get-AssetName {
    param([string]$t)
    if ($t -match "x86_64-pc-windows-msvc") { return "zeroclaw-x86_64-pc-windows-msvc.zip" }
    if ($t -match "x86_64-unknown-linux-gnu") { return "zeroclaw-x86_64-unknown-linux-gnu.tar.gz" }
    if ($t -match "aarch64-unknown-linux-gnu") { return "zeroclaw-aarch64-unknown-linux-gnu.tar.gz" }
    if ($t -match "x86_64-apple-darwin") { return "zeroclaw-x86_64-apple-darwin.tar.gz" }
    if ($t -match "aarch64-apple-darwin") { return "zeroclaw-aarch64-apple-darwin.tar.gz" }
    return $null
}

$AssetName = Get-AssetName $Target
if (-not $BuildFromSource -and $AssetName) {
    $Url = "$ZeroclawRepo/releases/download/$ReleaseTag/$AssetName"
    Write-Host "Downloading $Url ..."
    $TmpDir = Join-Path $env:TEMP "zeroclaw-dl-$([guid]::NewGuid().ToString('N').Substring(0,8))"
    New-Item -ItemType Directory -Force -Path $TmpDir | Out-Null
    try {
        $ZipPath = Join-Path $TmpDir (Split-Path $Url -Leaf)
        Invoke-WebRequest -Uri $Url -OutFile $ZipPath -UseBasicParsing
        if ($AssetName -match "\.zip$") {
            Expand-Archive -Path $ZipPath -DestinationPath $TmpDir -Force
            $Exe = Get-ChildItem -Path $TmpDir -Recurse -Filter "zeroclaw*.exe" | Select-Object -First 1
            if ($Exe) { Copy-Item $Exe.FullName $OutPath -Force }
        }
        if (Test-Path $OutPath) {
            Write-Host "Downloaded to $OutPath"
            exit 0
        }
    } catch {
        Write-Host "Download failed: $_"
    } finally {
        if (Test-Path $TmpDir) { Remove-Item -Recurse -Force $TmpDir -ErrorAction SilentlyContinue }
    }
}

# Fallback: build from source (submodule must be initialized)
$SubmodulePath = Join-Path $RepoRoot "crates\zeroclaw"
if (-not (Test-Path (Join-Path $SubmodulePath "Cargo.toml"))) {
    Write-Host "Submodule crates/zeroclaw not found. Run: git submodule update --init crates/zeroclaw"
    Write-Host "Or set ZEROCLAW_BUILD_FROM_SOURCE=0 and ensure a Release asset exists for $Target"
    exit 1
}
Write-Host "Building zeroclaw from source in $SubmodulePath ..."
Push-Location $SubmodulePath
try {
    & cargo build --release --bin zeroclaw 2>&1
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    $Built = Join-Path $SubmodulePath "target\release\zeroclaw$ExeExt"
    if (Test-Path $Built) {
        Copy-Item $Built $OutPath -Force
        Write-Host "Built and copied to $OutPath"
        exit 0
    }
} finally {
    Pop-Location
}
Write-Host "Failed to produce $OutPath"
exit 1
