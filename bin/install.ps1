# Mew Language Installer
# This script downloads the latest release of Mew language from GitHub,
# extracts it to ~/.mew, and adds it to your PATH environment variable

# Show a welcome message
Write-Host "   Mew Language Installer" -ForegroundColor Cyan
Write-Host "============================" -ForegroundColor Cyan

$Arch = [System.Environment]::GetEnvironmentVariable("PROCESSOR_ARCHITECTURE")
$Is64Bit = [System.Environment]::Is64BitOperatingSystem
$IsArm = $Arch -match "ARM"

$ArchFile = ""
if ($IsArm -and $Is64Bit) {
  $ArchFile = "windows-arm64"
}
elseif ($Is64Bit) {
  $ArchFile = "windows-x86_64"
}
else {
  $ArchFile = "windows-x86"
}

# Define installation directory
$installDir = Join-Path $env:USERPROFILE ".mew"
$zipPath = Join-Path $env:TEMP "mew-$ArchFile.zip"

# Create installation directory if it doesn't exist
if (-not (Test-Path $installDir)) {
  New-Item -ItemType Directory -Path $installDir -Force | Out-Null
}

# Get the latest release URL from GitHub API
try {
  $apiUrl = "https://api.github.com/repos/mewisme/mew-language/releases/latest"
  $release = Invoke-RestMethod -Uri $apiUrl -Headers @{
    "Accept"     = "application/vnd.github.v3+json"
    "User-Agent" = "Mew-Installer"
  }
    
  # Find the Windows x86_64 asset
  $windowsAsset = $release.assets | Where-Object { $_.name -eq "mew-windows-x86_64.zip" }
    
  if ($null -eq $windowsAsset) {
    throw "Could not find Windows x86_64 asset in the latest release"
  }
    
  $downloadUrl = $windowsAsset.browser_download_url
  $version = $release.tag_name
    
  Write-Host "Found Mew programming language version $version" -ForegroundColor Green
}
catch {
  Write-Host "Error fetching release information: $_" -ForegroundColor Red
  exit 1
}

# Download the release
try {
  Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath
}
catch {
  Write-Host "Error downloading Mew: $_" -ForegroundColor Red
  exit 1
}

Get-Process | Where-Object {
  $_.Modules.FileName -contains (Join-Path $installDir "mew.exe") 
} | Stop-Process -Force

# Remove old files from install directory (but keep any user files)
Remove-Item -Path (Join-Path $installDir "mew.exe") -Force -ErrorAction SilentlyContinue
Remove-Item -Path (Join-Path $installDir "README.md") -Force -ErrorAction SilentlyContinue
Remove-Item -Path (Join-Path $installDir "LICENSE") -Force -ErrorAction SilentlyContinue
Remove-Item -Path (Join-Path $installDir "examples") -Recurse -Force -ErrorAction SilentlyContinue

try {
  Expand-Archive -Path $zipPath -DestinationPath $installDir -Force
}
catch {
  Write-Host "Error extracting files: $_" -ForegroundColor Red
  exit 1
}

# Clean up the zip file
Remove-Item -Path $zipPath -Force

# Add to PATH if not already there
$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($userPath -notlike "*$installDir*") {
  [Environment]::SetEnvironmentVariable(
    "PATH", 
    "$userPath;$installDir", 
    "User"
  )
  $env:PATH = "$env:PATH;$installDir"
}

# Verify installation
$mewPath = Join-Path $installDir "mew.exe"
if (Test-Path $mewPath) {
  Write-Host ""
  Write-Host "Mew has been successfully installed to $installDir" -ForegroundColor Green
  try {
    $version = & $mewPath version 2>&1
    Write-Host "$version" -ForegroundColor Cyan
  }
  catch {
    Write-Host "Installed but couldn't verify version. You may need to restart your terminal." -ForegroundColor Yellow
  }
}
else {
  Write-Host "Installation may have failed. Could not find mew.exe in $installDir" -ForegroundColor Red
}

Write-Host "`nThank you for installing Mew!" -ForegroundColor Cyan
Write-Host "For help, type 'mew --help' in your terminal" -ForegroundColor Cyan