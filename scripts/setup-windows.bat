@echo off
REM Dev Storage Cleaner - Windows Setup Script
REM This script helps set up the required dependencies for building the app on Windows

echo ====================================
echo Dev Storage Cleaner - Windows Setup
echo ====================================
echo.

REM Check if Rust is installed
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Rust is not installed.
    echo.
    echo Please install Rust from: https://rustup.rs/
    echo.
    echo After installation:
    echo 1. Restart this terminal
    echo 2. Run this script again
    echo.
    pause
    exit /b 1
) else (
    echo [OK] Rust is already installed
    cargo --version
)

REM Check if Visual Studio Build Tools are available
where cl.exe >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [WARNING] Visual Studio Build Tools not found in PATH
    echo.
    echo You need Visual Studio Build Tools to compile Rust applications.
    echo.
    echo Please install one of the following:
    echo.
    echo 1. Visual Studio Build Tools (Recommended)
    echo    Download: https://visualstudio.microsoft.com/downloads/
    echo    Select: "Desktop development with C++" workload
    echo.
    echo 2. Visual Studio Community Edition
    echo    Download: https://visualstudio.microsoft.com/vs/community/
    echo    Select: "Desktop development with C++" workload
    echo.
    echo After installation:
    echo 1. Open "Developer Command Prompt for VS" or "x64 Native Tools Command Prompt"
    echo 2. Navigate to this directory
    echo 3. Run this script again
    echo.
    pause
    exit /b 1
) else (
    echo [OK] Visual Studio Build Tools are available
    cl.exe 2>nul | findstr /C:"Microsoft" >nul
)

echo.
echo ====================================
echo All dependencies are installed!
echo ====================================
echo.
echo Next steps:
echo 1. Build the project: cargo build --release
echo 2. Run the app: cargo run --release
echo.
echo Or simply run: build.bat
echo.
pause
