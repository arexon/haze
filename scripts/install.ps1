#!/usr/bin/env pwsh

$ErrorActionPreference = 'Stop'

$IsInstalled = $false
$ArchiveName = "haze-x86_64-pc-windows-msvc"
$DownloadUrl = "https://github.com/arexon/haze/releases/latest/download/${ArchiveName}.zip"
$BinDir = "${Home}\.haze"
$BinArchive = "${BinDir}\${ArchiveName}"

if (!(Test-Path $BinDir)) {
  New-Item $BinDir -ItemType Directory | Out-Null
} else {
  $IsInstalled = $true
}

curl.exe -Lo "${BinArchive}.zip" $DownloadUrl
tar.exe xf "${BinArchive}.zip" -C $BinDir
Move-Item -Path "${BinArchive}\*" -Destination $BinDir
Remove-Item $BinArchive
Remove-Item "${BinArchive}.zip"

$User = [System.EnvironmentVariableTarget]::User
$Path = [System.Environment]::GetEnvironmentVariable('Path', $User)
if (!(";${Path};".ToLower() -like "*;${BinDir};*".ToLower())) {
  [System.Environment]::SetEnvironmentVariable('Path', "${Path};${BinDir}", $User)
  $Env:Path += ";${BinDir}"
}

if ($IsInstalled -eq $true) {
  Write-Output "Haze was updated successfully to the latest version"
} else {
  Write-Output "Haze was installed successfully to ${BinDir}\haze.exe"
  Write-Output "Run `haze help` to get started!"
}
