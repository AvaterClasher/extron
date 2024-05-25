# PowerShell Script: download-and-install.ps1

# Define the URLs for the executables
$windowsUrl = "https://github.com/AvaterClasher/extron/releases/download/latest/extron.exe"

# Define the target path (typically a directory in the PATH)
$targetDir = [System.Environment]::GetEnvironmentVariable("PATH").Split(";") | Where-Object { Test-Path $_ -and (Get-Item $_).Attributes -band [System.IO.FileAttributes]::Directory } | Select-Object -First 1

# Determine the OS and download the appropriate executable
if ($IsWindows) {
    $exePath = Join-Path $targetDir "extron.exe"
    Invoke-WebRequest -Uri $windowsUrl -OutFile $exePath
    Write-Output "Downloaded and placed the executable at $exePath"
} else {
    Write-Output "This script is intended to run on Windows."
}
