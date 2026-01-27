# lilCopy Installer
# Uses HKEY_CURRENT_USER - no admin required

param(
    [switch]$Uninstall,
    [string]$InstallPath = "$env:LOCALAPPDATA\lilCopy"
)

$ErrorActionPreference = "Stop"

function Install-LilCopy {
    Write-Host "[*] Installing lilCopy..." -ForegroundColor Cyan
    
    # Create install directory
    if (-not (Test-Path $InstallPath)) {
        New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
    }
    
    # Copy executable
    $exePath = Join-Path $PSScriptRoot "target\release\lilcopy.exe"
    if (-not (Test-Path $exePath)) {
        Write-Host "[X] Error: lilcopy.exe not found. Run 'cargo build --release' first." -ForegroundColor Red
        exit 1
    }
    
    Copy-Item $exePath -Destination $InstallPath -Force
    $installedExe = Join-Path $InstallPath "lilcopy.exe"
    
    # Unblock the exe in case it was downloaded
    Unblock-File $installedExe -ErrorAction SilentlyContinue
    
    Write-Host "[+] Installed to: $InstallPath" -ForegroundColor Green
    
    # Register for all images via SystemFileAssociations
    $imageRegPath = "HKCU:\Software\Classes\SystemFileAssociations\image\shell\lilCopy"
    New-Item -Path $imageRegPath -Force | Out-Null
    Set-ItemProperty -Path $imageRegPath -Name "(default)" -Value "Copy Thumbnail"
    New-Item -Path "$imageRegPath\command" -Force | Out-Null
    Set-ItemProperty -Path "$imageRegPath\command" -Name "(default)" -Value "`"$installedExe`" `"%1`""
    Write-Host "    [+] Registered for all image types" -ForegroundColor DarkGray
    
    # Register for all videos via SystemFileAssociations
    $videoRegPath = "HKCU:\Software\Classes\SystemFileAssociations\video\shell\lilCopy"
    New-Item -Path $videoRegPath -Force | Out-Null
    Set-ItemProperty -Path $videoRegPath -Name "(default)" -Value "Copy Thumbnail"
    New-Item -Path "$videoRegPath\command" -Force | Out-Null
    Set-ItemProperty -Path "$videoRegPath\command" -Name "(default)" -Value "`"$installedExe`" `"%1`""
    Write-Host "    [+] Registered for all video types" -ForegroundColor DarkGray
    
    Write-Host ""
    Write-Host "[OK] lilCopy installed successfully!" -ForegroundColor Green
    Write-Host "     Right-click any image/video -> 'Copy Thumbnail'" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "     Restart Explorer or log out/in to see the context menu." -ForegroundColor Yellow
    Write-Host ""
    Write-Host "     Seize the memes of distribution, comrade!" -ForegroundColor Red
}

function Uninstall-LilCopy {
    Write-Host "[*] Uninstalling lilCopy..." -ForegroundColor Yellow
    
    # Remove image registration
    $imageRegPath = "HKCU:\Software\Classes\SystemFileAssociations\image\shell\lilCopy"
    if (Test-Path $imageRegPath) {
        Remove-Item -Path $imageRegPath -Recurse -Force
        Write-Host "    [-] Removed image handler" -ForegroundColor DarkGray
    }
    
    # Remove video registration
    $videoRegPath = "HKCU:\Software\Classes\SystemFileAssociations\video\shell\lilCopy"
    if (Test-Path $videoRegPath) {
        Remove-Item -Path $videoRegPath -Recurse -Force
        Write-Host "    [-] Removed video handler" -ForegroundColor DarkGray
    }
    
    # Clean up old HKCR registrations if they exist (from previous installs)
    $oldPaths = @(
        "Registry::HKEY_CLASSES_ROOT\*\shell\lilCopy",
        "Registry::HKEY_CLASSES_ROOT\jpegfile\shell\lilCopy",
        "Registry::HKEY_CLASSES_ROOT\pngfile\shell\lilCopy",
        "Registry::HKEY_CLASSES_ROOT\giffile\shell\lilCopy"
    )
    foreach ($path in $oldPaths) {
        if (Test-Path $path) {
            try {
                Remove-Item -Path $path -Recurse -Force -ErrorAction Stop
                Write-Host "    [-] Cleaned up old registration: $path" -ForegroundColor DarkGray
            } catch {
                Write-Host "    [!] Could not remove $path (may need admin)" -ForegroundColor Yellow
            }
        }
    }
    
    # Remove install directory
    if (Test-Path $InstallPath) {
        Remove-Item -Path $InstallPath -Recurse -Force
        Write-Host "    [-] Removed install directory" -ForegroundColor DarkGray
    }
    
    Write-Host ""
    Write-Host "[OK] lilCopy uninstalled." -ForegroundColor Green
}

# Main
if ($Uninstall) {
    Uninstall-LilCopy
} else {
    Install-LilCopy
}