# Version bump script for dev-storage-cleaner (Windows PowerShell)
# Usage: .\scripts\bump-version.ps1 [major|minor|patch]

param(
    [Parameter(Position=0)]
    [ValidateSet('major', 'minor', 'patch')]
    [string]$BumpType = 'patch'
)

$ErrorActionPreference = "Stop"

# Function to print colored output
function Write-Info {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Green
}

function Write-Warning-Custom {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor Yellow
}

function Write-Error-Custom {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

# Check if we're in the project root
if (-not (Test-Path "Cargo.toml")) {
    Write-Error-Custom "Cargo.toml not found. Please run this script from the project root."
    exit 1
}

# Check if git is available
try {
    git --version | Out-Null
} catch {
    Write-Error-Custom "git is not installed or not in PATH"
    exit 1
}

# Check if there are uncommitted changes
$gitStatus = git status --porcelain
if ($gitStatus) {
    Write-Warning-Custom "You have uncommitted changes. It's recommended to commit or stash them first."
    $response = Read-Host "Continue anyway? (y/N)"
    if ($response -ne 'y' -and $response -ne 'Y') {
        Write-Info "Aborted."
        exit 0
    }
}

# Get current version from Cargo.toml
$cargoContent = Get-Content "Cargo.toml" -Raw
if ($cargoContent -match 'version\s*=\s*"([^"]+)"') {
    $currentVersion = $matches[1]
} else {
    Write-Error-Custom "Could not extract current version from Cargo.toml"
    exit 1
}

Write-Info "Current version: $currentVersion"

# Parse version
$versionParts = $currentVersion -split '\.'
$major = [int]$versionParts[0]
$minor = [int]$versionParts[1]
$patch = [int]$versionParts[2]

# Determine bump type
switch ($BumpType) {
    'major' {
        $major++
        $minor = 0
        $patch = 0
    }
    'minor' {
        $minor++
        $patch = 0
    }
    'patch' {
        $patch++
    }
}

$newVersion = "$major.$minor.$patch"
Write-Info "New version: $newVersion"

# Confirm before proceeding
$response = Read-Host "Proceed with version bump? (y/N)"
if ($response -ne 'y' -and $response -ne 'Y') {
    Write-Info "Aborted."
    exit 0
}

# Update Cargo.toml
Write-Info "Updating Cargo.toml..."
$cargoContent = Get-Content "Cargo.toml" -Raw
$cargoContent = $cargoContent -replace "version\s*=\s*`"$currentVersion`"", "version = `"$newVersion`""
Set-Content "Cargo.toml" -Value $cargoContent -NoNewline

# Update Cargo.lock
Write-Info "Updating Cargo.lock..."
try {
    cargo check 2>&1 | Out-Null
} catch {
    # Ignore errors
}

# Git operations
Write-Info "Creating git commit and tag..."
git add Cargo.toml Cargo.lock
git commit -m "Bump version to $newVersion"

$tagName = "v$newVersion"
git tag -a "$tagName" -m "Release $newVersion"

Write-Info "Version bumped successfully!"
Write-Host ""
Write-Info "Summary:"
Write-Host "  Old version: $currentVersion"
Write-Host "  New version: $newVersion"
Write-Host "  Tag created: $tagName"
Write-Host ""
Write-Info "Next steps:"
Write-Host "  1. Review the commit: git show HEAD"
Write-Host "  2. Push changes: git push origin main"
Write-Host "  3. Push tag: git push origin $tagName"
Write-Host ""
Write-Warning-Custom "The GitHub Actions workflow will automatically create a release when you push the tag."
