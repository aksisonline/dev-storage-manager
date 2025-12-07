#!/bin/bash

# Version bump script for dev-storage-cleaner
# Usage: ./scripts/bump-version.sh [major|minor|patch]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the project root
if [ ! -f "Cargo.toml" ]; then
    print_error "Cargo.toml not found. Please run this script from the project root."
    exit 1
fi

# Check if git is available
if ! command -v git &> /dev/null; then
    print_error "git is not installed or not in PATH"
    exit 1
fi

# Check if there are uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
    print_warning "You have uncommitted changes. It's recommended to commit or stash them first."
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_info "Aborted."
        exit 0
    fi
fi

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep "^version" Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

if [ -z "$CURRENT_VERSION" ]; then
    print_error "Could not extract current version from Cargo.toml"
    exit 1
fi

print_info "Current version: $CURRENT_VERSION"

# Parse version
IFS='.' read -r -a VERSION_PARTS <<< "$CURRENT_VERSION"
MAJOR="${VERSION_PARTS[0]}"
MINOR="${VERSION_PARTS[1]}"
PATCH="${VERSION_PARTS[2]}"

# Determine bump type
BUMP_TYPE="${1:-patch}"

case "$BUMP_TYPE" in
    major)
        MAJOR=$((MAJOR + 1))
        MINOR=0
        PATCH=0
        ;;
    minor)
        MINOR=$((MINOR + 1))
        PATCH=0
        ;;
    patch)
        PATCH=$((PATCH + 1))
        ;;
    *)
        print_error "Invalid bump type: $BUMP_TYPE"
        echo "Usage: $0 [major|minor|patch]"
        exit 1
        ;;
esac

NEW_VERSION="$MAJOR.$MINOR.$PATCH"
print_info "New version: $NEW_VERSION"

# Confirm before proceeding
read -p "Proceed with version bump? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_info "Aborted."
    exit 0
fi

# Update Cargo.toml
print_info "Updating Cargo.toml..."
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    sed -i '' "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml
else
    # Linux
    sed -i "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml
fi

# Update Cargo.lock
print_info "Updating Cargo.lock..."
cargo check --quiet 2>/dev/null || true

# Git operations
print_info "Creating git commit and tag..."
git add Cargo.toml Cargo.lock
git commit -m "Bump version to $NEW_VERSION"

TAG_NAME="v$NEW_VERSION"
git tag -a "$TAG_NAME" -m "Release $NEW_VERSION"

print_info "Version bumped successfully!"
echo ""
print_info "Summary:"
echo "  Old version: $CURRENT_VERSION"
echo "  New version: $NEW_VERSION"
echo "  Tag created: $TAG_NAME"
echo ""
print_info "Next steps:"
echo "  1. Review the commit: git show HEAD"
echo "  2. Push changes: git push origin main"
echo "  3. Push tag: git push origin $TAG_NAME"
echo ""
print_warning "The GitHub Actions workflow will automatically create a release when you push the tag."
