# Download uninstall tool
$releases = Invoke-RestMethod -Uri "https://api.github.com/repos/dotnet/cli-lab/releases/latest"
$asset = $releases.assets | Where-Object { $_.name -eq "dotnet-core-uninstall.msi" } | Select-Object -First 1
$url = $asset.browser_download_url
Invoke-WebRequest -Uri $url -OutFile $(Split-Path $url -Leaf)

# Prepare uninstall tool
$pwd = (Get-Location).Path
msiexec.exe /A dotnet-core-uninstall.msi TARGETDIR=$pwd /QN /L*V log.txt
$uninstallToolPath = Join-Path $pwd "dotnet-core-uninstall\dotnet-core-uninstall-tool.exe"

# Perform uninstall
$uninstallToolPath remove --all
