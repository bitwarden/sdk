$ErrorActionPreference = "Stop"

$bwsVersion = if ($env:bwsVersion) { $env:bwsVersion } else { "0.5.0" }
$installDir = [Environment]::GetFolderPath([Environment+SpecialFolder]::LocalApplicationData) | Join-Path -ChildPath "Programs" | Join-Path -ChildPath "Bitwarden"

function Test-BwsInstallation {
  $existingBws = Get-Command bws -ErrorAction SilentlyContinue
  if ($null -ne $existingBws) {
    $userInput = Read-Host "bws is already installed at $($existingBws.Source). Do you want to overwrite it? (Y/N)"
    if ($userInput -ne "Y") {
      Write-Host "Installation cancelled by user."
      exit
    }
  }
}

function Invoke-BwsDownload {
  # https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-processor#properties
  $processorArch = (Get-CimInstance -ClassName Win32_Processor).Architecture
  Write-Host "Detected architecture: $processorArch"
  if ($processorArch -eq 9) {
    $arch = "x86_64"
  } elseif ($processorArch -eq 12) {
    $arch = "aarch64"
  } else {
    throw "Unsupported architecture: $processorArch"
  }

  $bwsUrl = "https://github.com/bitwarden/sdk/releases/download/bws-v$bwsVersion/bws-$arch-pc-windows-msvc-$bwsVersion.zip"
  Write-Host "Downloading bws from: $bwsUrl"
  $outputPath = Join-Path $env:TEMP "bws.zip"
  Invoke-WebRequest -Uri $bwsUrl -OutFile $outputPath
  return $outputPath
}

function Install-Bws {
  param($zipPath)
  Write-Host "Installing bws..."
  New-Item -ItemType Directory -Force -Path $installDir | Out-Null
  Expand-Archive -Force $zipPath $installDir
  Write-Host "bws installed to $installDir"
  setx PATH "$env:PATH;$installDir"
  Write-Host "$installDir has been added to your PATH"
  Write-Host "Please restart your shell to use bws"
}

function Test-Bws {
  Write-Host "Checking bws..."
  $bwsPath = Join-Path $installDir "bws.exe"
  if (Test-Path $bwsPath) {
    Write-Host "bws is installed at $bwsPath"
  } else {
    throw "bws is not installed"
  }
}

Test-BwsInstallation
$zipPath = Invoke-BwsDownload
Install-Bws -zipPath $zipPath
Test-Bws
