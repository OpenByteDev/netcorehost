# Download uninstall tool
$releases = Invoke-RestMethod -Uri "https://api.github.com/repos/dotnet/cli-lab/releases/latest"
$asset = $releases.assets | Where-Object { $_.name -eq "dotnet-core-uninstall.msi" } | Select-Object -First 1
$url = $asset.browser_download_url
Invoke-WebRequest -Uri $url -OutFile $(Split-Path $url -Leaf)

# Prepare uninstall tool
$extractPath = Join-Path $pwd "dotnet-core-uninstall" # needs to be a new path
msiexec.exe /A dotnet-core-uninstall.msi TARGETDIR=$extractPath /QN /L*V log.txt
$uninstallToolPath = Join-Path $extractPath "dotnet-core-uninstall" "dotnet-core-uninstall.exe"
# wait for the tool to be ready
$maxRetries = 30
$retry = 0
while (-not (Test-Path $uninstallToolPath) -and ($retry -lt $maxRetries)) {
    Start-Sleep -Seconds 1
    $retry++
}
if ($retry -eq $maxRetries) {
    Write-Error "Uninstall tool was not found after $maxRetries seconds."
    Get-Content -Path "log.txt" | Write-Host
    exit 1
}

# Perform uninstall
& $uninstallToolPath remove --yes --force --all --aspnet-runtime --verbosity detailed
& $uninstallToolPath remove --yes --force --all --hosting-bundle --verbosity detailed
& $uninstallToolPath remove --yes --force --all --runtime --verbosity detailed
& $uninstallToolPath remove --yes --force --all --sdk --verbosity detailed
