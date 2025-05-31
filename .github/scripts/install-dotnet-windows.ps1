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

Add-Content -Path $Env:GITHUB_PATH -Value $dotnetRoot
Add-Content -Path $Env:GITHUB_ENV -Value "DOTNET_ROOT=$dotnetRoot"
