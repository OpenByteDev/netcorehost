param (
    [string]$Version,
    [string]$Arch
)

Invoke-WebRequest -Uri https://dot.net/v1/dotnet-install.ps1 -OutFile dotnet-install.ps1
./dotnet-install.ps1 -Architecture $Arch -Channel $Version

if ($Env:DOTNET_INSTALL_DIR) {
    $dotnetRoot = $Env:DOTNET_INSTALL_DIR
} else {
    $dotnetRoot = Join-Path $Env:LOCALAPPDATA "Microsoft\dotnet"
}

$dotnetRoot >> $Env:GITHUB_PATH
"DONET_ROOT=$dotnetRoot" >> $Env:GITHUB_ENV
