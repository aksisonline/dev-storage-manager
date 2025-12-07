@echo off
REM Dev Storage Cleaner - Windows Build Script
REM This script builds the application for release

echo ====================================
echo Building Dev Storage Cleaner...
echo ====================================
echo.

REM Check if cargo is installed
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Cargo is not installed.
    echo Please run setup-windows.bat first or install Rust from https://rustup.rs/
    pause
    exit /b 1
)

REM Check if Visual Studio Build Tools are available
where cl.exe >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [WARNING] Visual Studio Build Tools not found in PATH
    echo.
    echo Please run this script from:
    echo - Developer Command Prompt for VS
    echo - x64 Native Tools Command Prompt for VS
    echo.
    echo Or run setup-windows.bat for more information.
    pause
    exit /b 1
)

echo Building release binary...
echo.
cargo build --release

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ====================================
    echo Build successful!
    echo ====================================
    echo.
    echo Binary location:
    echo   target\release\dev-storage-cleaner.exe
    echo.
    echo To run the app:
    echo   .\target\release\dev-storage-cleaner.exe
    echo   or
    echo   cargo run --release
    echo.
) else (
    echo.
    echo ====================================
    echo Build failed!
    echo ====================================
    echo Please check the error messages above.
    echo.
    pause
    exit /b 1
)

pause
