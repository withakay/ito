# Ito installer for Windows (PowerShell)
#
# Downloads the latest Windows release from GitHub and installs `ito.exe` into a
# user-writable directory.
#
# Usage (recommended: download, inspect, then run):
#   powershell -ExecutionPolicy Bypass -File .\install.ps1
#
# Optional:
#   -Version <vX.Y.Z|X.Y.Z>   Install a specific release tag
#   -InstallDir <path>        Override install directory
#   -AddToPath                Append InstallDir to the user PATH

[CmdletBinding()]
param(
  [string]$Version = "",
  [string]$InstallDir = "",
  [switch]$AddToPath
)

$ErrorActionPreference = "Stop"

$Repo = "withakay/ito"

function Fail([string]$Message) {
  Write-Error $Message
  exit 1
}

function Get-LatestTag([string]$RepoName) {
  $url = "https://api.github.com/repos/$RepoName/releases/latest"
  $res = Invoke-RestMethod -Uri $url -Headers @{ "User-Agent" = "ito-install" }
  if (-not $res.tag_name) {
    Fail "failed to determine latest release tag"
  }
  return [string]$res.tag_name
}

function Normalize-Tag([string]$Tag) {
  if ([string]::IsNullOrWhiteSpace($Tag)) {
    return ""
  }
  if ($Tag.StartsWith("v")) {
    return $Tag
  }
  return "v$Tag"
}

function Get-WindowsTarget() {
  $arch = $env:PROCESSOR_ARCHITECTURE
  if ($arch -eq "AMD64") {
    return "x86_64-pc-windows-msvc"
  }
  Fail "unsupported Windows arch: $arch (expected AMD64)"
}

function Download-File([string]$Url, [string]$OutFile) {
  Invoke-WebRequest -Uri $Url -OutFile $OutFile -Headers @{ "User-Agent" = "ito-install" } | Out-Null
}

function Read-ExpectedSha256([string]$ChecksumFile) {
  $line = (Get-Content -Path $ChecksumFile -TotalCount 1)
  if ([string]::IsNullOrWhiteSpace($line)) {
    Fail "checksum file was empty"
  }
  return ($line -split "\s+")[0]
}

function Ensure-PathContains([string]$Dir) {
  $current = [Environment]::GetEnvironmentVariable("PATH", "User")
  if (-not $current) {
    $current = ""
  }

  $parts = $current -split ";" | ForEach-Object { $_.Trim() } | Where-Object { $_ -ne "" }
  foreach ($p in $parts) {
    if ($p -ieq $Dir) {
      return
    }
  }

  $newPath = $current
  if (-not [string]::IsNullOrWhiteSpace($newPath)) {
    $newPath = "$newPath;"
  }
  $newPath = "$newPath$Dir"
  [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
}

try {
  $tag = ""
  if (-not [string]::IsNullOrWhiteSpace($Version)) {
    $tag = Normalize-Tag $Version
  } else {
    $tag = Normalize-Tag (Get-LatestTag $Repo)
  }

  if ([string]::IsNullOrWhiteSpace($tag)) {
    Fail "failed to determine release tag"
  }

  $target = Get-WindowsTarget
  $archive = "ito-$tag-$target.zip"
  $checksum = "ito-$tag-$target.sha256"
  $baseUrl = "https://github.com/$Repo/releases/download/$tag"

  if ([string]::IsNullOrWhiteSpace($InstallDir)) {
    $InstallDir = Join-Path $env:LOCALAPPDATA "ito\bin"
  }

  $tmpRoot = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid().ToString("n"))
  New-Item -ItemType Directory -Path $tmpRoot | Out-Null

  $archivePath = Join-Path $tmpRoot $archive
  $checksumPath = Join-Path $tmpRoot $checksum

  Download-File "$baseUrl/$archive" $archivePath
  Download-File "$baseUrl/$checksum" $checksumPath

  $expected = (Read-ExpectedSha256 $checksumPath).ToLowerInvariant()
  $actual = (Get-FileHash -Algorithm SHA256 -Path $archivePath).Hash.ToLowerInvariant()
  if ($expected -ne $actual) {
    Fail "checksum mismatch for $archive`nexpected: $expected`nactual:   $actual"
  }

  $extractDir = Join-Path $tmpRoot "extract"
  Expand-Archive -Path $archivePath -DestinationPath $extractDir -Force

  $exe = Get-ChildItem -Path $extractDir -Recurse -File -Filter "ito.exe" | Select-Object -First 1
  if (-not $exe) {
    Fail "archive did not contain expected binary: ito.exe"
  }

  New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
  $dest = Join-Path $InstallDir "ito.exe"
  Copy-Item -Path $exe.FullName -Destination $dest -Force

  Write-Host "installed ito to $dest"
  & $dest --version | Out-Host

  if ($AddToPath) {
    Ensure-PathContains $InstallDir
    Write-Host "added '$InstallDir' to user PATH (open a new terminal)"
  } else {
    Write-Host "note: '$InstallDir' is not on PATH by default"
  }
}
finally {
  if ($tmpRoot -and (Test-Path $tmpRoot)) {
    Remove-Item -Recurse -Force $tmpRoot
  }
}
