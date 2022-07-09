$dotnetInstallDir = Get-Variable DOTNET_INSTALL_DIR -ErrorAction SilentlyContinue
Write-Output "DOTNET=$dotnetInstallDir"
if ($dotnetInstallDir) {
    Write-Output $dotnetInstallDir >> $GITHUB_PATH
    Write-Output "DOTNET_ROOT=$dotnetInstallDir" >> $GITHUB_ENV
} else {
    if ($IsWindows) {
        $localAppData = Get-Variable LocalAppData
        $dotnetRoot = Join-Path $localAppData "Microsoft" "dotnet"
    } else {
        $homeVar = Get-Variable HOME
        $dotnetRoot = Join-Path $homeVar ".dotnet"
    }
        
    Write-Output $dotnetRoot >> $GITHUB_PATH
    Write-Output "DOTNET_ROOT=$dotnetRoot" >> $GITHUB_ENV
}
