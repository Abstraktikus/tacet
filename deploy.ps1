# Bundles Tacet in release mode and copies the VST3 into the system VST3 folder.
# Run from an ELEVATED PowerShell (writing to Program Files needs admin), or set
# $VstDir to a user-writable folder that GigPerformer is configured to scan.
#
# The plugin must match the HOST's architecture. On this machine GigPerformer 5 is
# x64, so we build for x86_64 even though the native Rust toolchain is ARM64.
# Override with e.g. -Target aarch64-pc-windows-msvc for a native ARM64 host.

param(
    [string]$Target = 'x86_64-pc-windows-msvc',
    [string]$VstDir = 'C:\Program Files\Common Files\VST3'
)

$ErrorActionPreference = 'Stop'

$Source = Join-Path $PSScriptRoot 'target\bundled\Tacet.vst3'

Write-Host "Bundling Tacet (release, $Target)..."
cargo xtask bundle tacet --release --target $Target
if ($LASTEXITCODE -ne 0) { throw "bundle failed with exit code $LASTEXITCODE" }

if (-not (Test-Path $Source)) { throw "bundle not found at $Source" }
if (-not (Test-Path $VstDir)) { New-Item -ItemType Directory -Path $VstDir | Out-Null }

$Dest = Join-Path $VstDir 'Tacet.vst3'
if (Test-Path $Dest) { Remove-Item -Recurse -Force $Dest }
Copy-Item -Recurse -Force $Source $Dest

Write-Host "Deployed Tacet.vst3 to $VstDir"
