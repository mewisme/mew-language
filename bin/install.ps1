#!/usr/bin/env pwsh

$ErrorActionPreference = "Stop"

$Arch = [System.Environment]::GetEnvironmentVariable("PROCESSOR_ARCHITECTURE")
$Is64Bit = [System.Environment]::Is64BitOperatingSystem
$IsArm = $Arch -match "ARM"

$ArchFile = ""
if ($IsArm -and $Is64Bit) {
    $ArchFile = "windows-arm64"
} elseif ($Is64Bit) {
    $ArchFile = "windows-x86_64"
} else {
    $ArchFile = "windows-x86"
}

Write-Host "   Mew Language Installer" -ForegroundColor Cyan
Write-Host "============================" -ForegroundColor Cyan

$LatestRelease = Invoke-RestMethod -Uri "https://api.github.com/repos/MewTheDev/mew/releases/latest"
$Version = $LatestRelease.tag_name -replace "^v", ""

$DownloadUrl = "https://github.com/MewTheDev/mew/releases/download/v$Version/mew-$ArchFile.zip"
$TempPath = Join-Path ([System.IO.Path]::GetTempPath()) "mew-$Version-$ArchFile.zip"
$InstallDir = Join-Path $env:LOCALAPPDATA "Mew"

Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempPath

if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir | Out-Null
    Write-Host "Created installation directory: $InstallDir"
}

Expand-Archive -Path $TempPath -DestinationPath $InstallDir -Force

$UserPath = [System.Environment]::GetEnvironmentVariable("Path", "User")
if (-not $UserPath.Contains($InstallDir)) {
    [System.Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
}

Remove-Item $TempPath

Write-Host "`nThank you for installing Mew!" -ForegroundColor Cyan
Write-Host "For help, type 'mew --help' in your terminal" -ForegroundColor Cyan
