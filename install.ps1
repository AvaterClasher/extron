[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$ProgressPreference = 'SilentlyContinue'
$release = Invoke-RestMethod -Method Get -Uri "https://api.github.com/repos/AvaterClasher/extron/releases/latest"
$asset = $release.assets | Where-Object { $_.name -like "*extron*.exe" }
$destdir = "$home\bin"
$exeFile = "$env:TEMP\$($asset.name)"

Write-Output "Downloading: $($asset.name)"
Invoke-RestMethod -Method Get -Uri $asset.browser_download_url -OutFile $exeFile

$extronPath = "${destdir}\extron.exe"

if (Test-Path -Path $extronPath -PathType Leaf) {
    Write-Output "Removing previous installation of extron from $destdir"
    Remove-Item -r -fo $extronPath
}

# Create dir for exe file if it doesn't exist
New-Item -ItemType Directory -Path $destdir -Force | Out-Null

# Move the downloaded exe to the destination directory
Move-Item -Path $exeFile -Destination $extronPath

# Copy xh.exe as xhs.exe into bin
# Copy-Item $xhPath $xhsPath

# Inform user where the executables have been put
Write-Output "Extron has been installed to:`n - $extronPath"

# Make sure destdir is in the path
$userPath = [System.Environment]::GetEnvironmentVariable('Path', [System.EnvironmentVariableTarget]::User)
$machinePath = [System.Environment]::GetEnvironmentVariable('Path', [System.EnvironmentVariableTarget]::Machine)

# If userPath AND machinePath both do not contain bin, then add it to user path
if (!($userPath.ToLower().Contains($destdir.ToLower())) -and !($machinePath.ToLower().Contains($destdir.ToLower()))) {
    # Update userPath
    $userPath = $userPath.Trim(";") + ";$destdir"

    # Modify PATH for new windows
    Write-Output "`nAdding $destdir directory to the PATH variable."
    [System.Environment]::SetEnvironmentVariable('Path', $userPath, [System.EnvironmentVariableTarget]::User)

    # Modify PATH for current terminal
    Write-Output "`nRefreshing current terminal's PATH for you."
    $Env:Path = $Env:Path.Trim(";") + ";$destdir"

    # Instruct how to modify PATH for other open terminals
    Write-Output "`nFor other terminals, restart them (or the entire IDE if they're within one).`n"
}
