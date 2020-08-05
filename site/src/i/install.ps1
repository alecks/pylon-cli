#!/usr/bin/env pwsh
# Copy, paste, replace from Deno's install script at https://github.com/denoland/deno_install/blob/master/install.ps1
# Copyright 2018 the Deno authors. All rights reserved. MIT license.
# Keep this script simple and easily auditable.

$ErrorActionPreference = 'Stop'

if ($args.Length -gt 0) {
  $Version = $args.Get(0)
}

if ($PSVersionTable.PSEdition -ne 'Core') {
  $IsWindows = $true
  $IsMacOS = $false
}

$PylonInstall = $env:PYLON_INSTALL
$BinDir = if ($PylonInstall) {
  "$PylonInstall\bin"
} elseif ($IsWindows) {
  "$Home\.pylon\bin"
}
$PylonExe = "$BinDir\pylon.exe"
$Target = 'windows'

# GitHub requires TLS 1.2
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$PylonUri = if (!$Version) {
  $Response = Invoke-WebRequest 'https://github.com/fjah/pylon-cli/releases' -UseBasicParsing
  if ($PSVersionTable.PSEdition -eq 'Core') {
    $Response.Links |
      Where-Object { $_.href -like "/fjah/pylon-cli/releases/download/*/pylon-cli-${Target}.exe" } |
      ForEach-Object { 'https://github.com' + $_.href } |
      Select-Object -First 1
  } else {
    $HTMLFile = New-Object -Com HTMLFile
    if ($HTMLFile.IHTMLDocument2_write) {
      $HTMLFile.IHTMLDocument2_write($Response.Content)
    } else {
      $ResponseBytes = [Text.Encoding]::Unicode.GetBytes($Response.Content)
      $HTMLFile.write($ResponseBytes)
    }
    $HTMLFile.getElementsByTagName('a') |
      Where-Object { $_.href -like "about:/fjah/pylon-cli/releases/download/*/pylon-${Target}.exe" } |
      ForEach-Object { $_.href -replace 'about:', 'https://github.com' } |
      Select-Object -First 1
  }
} else {
  "https://github.com/fjah/pylon-cli/releases/download/$Version/pylon-cli-${Target}.exe"
}

if (!(Test-Path $BinDir)) {
  New-Item $BinDir -ItemType Directory | Out-Null
}

Invoke-WebRequest $PylonUri -OutFile $PylonExe -UseBasicParsing

$User = [EnvironmentVariableTarget]::User
$Path = [Environment]::GetEnvironmentVariable('Path', $User)
if (!(";$Path;".ToLower() -like "*;$BinDir;*".ToLower())) {
  [Environment]::SetEnvironmentVariable('Path', "$Path;$BinDir", $User)
  $Env:Path += ";$BinDir"
}
Write-Output "pylon-cli was installed successfully to $PylonExe"
Write-Output "Run 'pylon --help' to get started"