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

[Environment]::SetEnvironmentVariable("PATH", [Environment]::GetEnvironmentVariable("PATH", [EnvironmentVariableTarget]::Process) + ";$dotnetRoot", [EnvironmentVariableTarget]::Process)
[Environment]::SetEnvironmentVariable("DOTNET_ROOT", $dotnetRoot, [EnvironmentVariableTarget]::Process)

# $env:PATH += ";$dotnetRoot"
# $env:DOTNET_ROOT=$dotnetRoot

$dotnetRoot >> $GITHUB_PATH
"DOTNET_ROOT=$dotnetRoot" >> $GITHUB_ENV

Write-Output "DOTNET_ROOT=$dotnetRoot"
