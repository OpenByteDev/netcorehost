# Download uninstall tool
$headers = @{
    Authorization = "Bearer $($env:GITHUB_TOKEN)"
}
$releases = Invoke-RestMethod -Uri "https://api.github.com/repos/dotnet/cli-lab/releases/latest" -Headers $headers
$asset = $releases.assets | Where-Object { $_.name -eq "dotnet-core-uninstall.msi" } | Select-Object -First 1

# Check if the asset is missing
if (-not $asset) {
    throw "Release asset 'dotnet-core-uninstall.msi' was not found in the latest release."
}
if (-not $asset.browser_download_url) {
    throw "Release asset 'dotnet-core-uninstall.msi' does not have a browser_download_url."
}

$url = $asset.browser_download_url
Invoke-WebRequest -Uri $url -OutFile $(Split-Path $url -Leaf)

# Prepare uninstall tool
$extractPath = Join-Path $pwd "dotnet-core-uninstall" # needs to be a new path

# Run msiexec
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
    if (Test-Path "log.txt") { Get-Content -Path "log.txt" | Write-Host }

    Write-Host "Expected: $uninstallToolPath"
    Write-Host "extractPath: $extractPath"
    Write-Host "extractPath exists? $(Test-Path $extractPath)"

    Write-Host "Top-level contents of extractPath:"
    if (Test-Path $extractPath) {
        Get-ChildItem -Path $extractPath -Force -ErrorAction SilentlyContinue |
            Select-Object FullName | Out-String | Write-Host
    }

    Write-Host "Searching for dotnet-core-uninstall.exe under extractPath (diagnostic only):"
    if (Test-Path $extractPath) {
        Get-ChildItem -Path $extractPath -Recurse -Filter "dotnet-core-uninstall.exe" -Force -ErrorAction SilentlyContinue |
            Select-Object FullName | Out-String | Write-Host
    }

    exit 1
}

# Perform uninstall
& $uninstallToolPath remove --yes --force --all --runtime --verbosity detailed
& $uninstallToolPath remove --yes --force --all --sdk --verbosity detailed
