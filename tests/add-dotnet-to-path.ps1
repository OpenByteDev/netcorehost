#!/usr/bin/env pwsh

if ($Env:DOTNET_INSTALL_DIR) {
    $dotnetRoot = $Env:DOTNET_INSTALL_DIR
} else {
    if ([System.Environment]::OSVersion.Platform -eq "Win32NT") {
        $dotnetRoot = [IO.Path]::Combine($Env:LOCALAPPDATA, "Microsoft", "dotnet")
    } else {
        $dotnetRoot = [IO.Path]::Combine($Env:HOME, ".dotnet")
    }
}

$dotnetRoot >> $GITHUB_PATH
"DOTNET_ROOT=$dotnetRoot" >> $GITHUB_ENV

Write-Output "DOTNET_ROOT=$dotnetRoot"
Write-Output "GITHUB_PATH=$GITHUB_PATH"
Write-Output "GITHUB_ENV=$GITHUB_ENV"
