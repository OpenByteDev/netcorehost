Write-Output "Starting .NET uninstall on Windows..."

$uninstallToolPath = "C:\Program Files\dotnet-core-uninstall\dotnet-core-uninstall.exe"
$uninstallToolDownloadUrl = "https://aka.ms/dotnet-core-uninstall-tool-win"

# Install uninstall tool
Write-Output "Downloading .NET Uninstall Tool..."
$zipPath = "$env:TEMP\dotnet-core-uninstall-tool.zip"
Invoke-WebRequest -Uri $uninstallToolDownloadUrl -OutFile $zipPath

Write-Output "Extracting..."
Expand-Archive -Path $zipPath -DestinationPath "C:\Program Files\dotnet-core-uninstall" -Force
Remove-Item $zipPath

# Perform uninstall
Write-Output "Removing all SDKs and runtimes..."
& "$uninstallToolPath" remove --all

Write-Output ".NET uninstall process completed."
