@echo off
REM Dev Storage Cleaner - Windows Packaging Script
REM This script packages the Windows build into a distributable zip file

echo ====================================
echo Dev Storage Cleaner - Windows Packager
echo ====================================
echo.

REM Check if the release binary exists
if not exist "target\release\dev-storage-cleaner.exe" (
    echo [ERROR] Release binary not found!
    echo.
    echo Please build the release version first:
    echo   cargo build --release
    echo.
    pause
    exit /b 1
)

echo [OK] Found release binary
echo.

REM Create distribution directory
echo Creating distribution package...
if exist "dist" (
    rmdir /s /q dist
)
mkdir dist

REM Copy the executable
echo Copying executable...
copy target\release\dev-storage-cleaner.exe dist\

REM Copy documentation
if exist "README.md" (
    echo Copying README.md...
    copy README.md dist\
)

if exist "QUICKSTART.md" (
    echo Copying QUICKSTART.md...
    copy QUICKSTART.md dist\
)

if exist "USAGE.md" (
    echo Copying USAGE.md...
    copy USAGE.md dist\
)

if exist "LICENSE" (
    echo Copying LICENSE...
    copy LICENSE dist\
)

REM Create a simple launcher batch file
echo Creating launcher...
echo @echo off > dist\run.bat
echo echo Starting Dev Storage Cleaner... >> dist\run.bat
echo start dev-storage-cleaner.exe >> dist\run.bat

echo.
echo ====================================
echo Package created successfully!
echo ====================================
echo.
echo Location: %CD%\dist
echo.
echo Contents:
dir /b dist
echo.

REM Calculate approximate size
for /f "tokens=3" %%a in ('dir /s dist ^| find "File(s)"') do set size=%%a
echo Total size: %size% bytes
echo.

REM Ask if user wants to create a zip file
echo Would you like to create a zip file? (Y/N)
set /p CREATE_ZIP=

if /i "%CREATE_ZIP%"=="Y" (
    echo.
    echo Creating zip file...

    REM Use PowerShell to create zip (available on Windows 10+)
    powershell -command "Compress-Archive -Path dist\* -DestinationPath DevStorageCleaner-Windows.zip -Force"

    if exist "DevStorageCleaner-Windows.zip" (
        echo.
        echo [OK] Zip file created: DevStorageCleaner-Windows.zip
        for %%A in (DevStorageCleaner-Windows.zip) do echo Size: %%~zA bytes
    ) else (
        echo.
        echo [ERROR] Failed to create zip file
    )
)

echo.
echo Distribution package is ready!
echo.
echo To distribute:
echo   1. Share the 'dist' folder, or
echo   2. Share the 'DevStorageCleaner-Windows.zip' file
echo.
echo To install on another PC:
echo   1. Copy the contents of 'dist' to any folder
echo   2. Run 'dev-storage-cleaner.exe' or 'run.bat'
echo.
pause
