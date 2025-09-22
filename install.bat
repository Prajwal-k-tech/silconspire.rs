@echo off
REM Install script for silconspire - Windows

echo Installing SiliconSpire QAP Solver...

REM Create temp directory
set TEMP_DIR=%TEMP%\silconspire-install
mkdir "%TEMP_DIR%" 2>nul
cd /d "%TEMP_DIR%"

REM Download latest release
echo Downloading latest release...
powershell -Command "& {$repo='Prajwal-k-tech/silconspire.rs'; $latest=(Invoke-RestMethod -Uri 'https://api.github.com/repos/$repo/releases/latest'); $url=$latest.assets | Where-Object {$_.name -like '*windows*'} | Select-Object -First 1 -ExpandProperty browser_download_url; Invoke-WebRequest -Uri $url -OutFile 'silconspire.exe'}"

if %errorlevel% neq 0 (
    echo Failed to download binary
    exit /b 1
)

REM Install to Program Files
set INSTALL_DIR=%ProgramFiles%\SiliconSpire
mkdir "%INSTALL_DIR%" 2>nul
copy silconspire.exe "%INSTALL_DIR%\silconspire.exe"

REM Add to PATH (requires admin)
powershell -Command "Start-Process cmd -ArgumentList '/c setx PATH \"%PATH%;%INSTALL_DIR%\" /M' -Verb RunAs"

echo.
echo Installation complete!
echo.
echo Usage:
echo   silconspire --help
echo   silconspire --input-file my_problem.txt --pack-size 50
echo.
echo You may need to restart your command prompt for PATH changes to take effect.
echo For more information, visit: https://github.com/Prajwal-k-tech/silconspire.rs

pause
