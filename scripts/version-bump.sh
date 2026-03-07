#!/usr/bin/env bash
# version-bump.sh - Bump version across all project files and create git tag
# Usage: ./scripts/version-bump.sh major|minor|patch
set -euo pipefail

NEXUS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$NEXUS_DIR"

if [ $# -lt 1 ]; then
  echo "Usage: $0 major|minor|patch"
  echo ""
  echo "Updates version in:"
  echo "  - package.json"
  echo "  - src-tauri/Cargo.toml"
  echo "  - src-tauri/tauri.conf.json"
  echo ""
  echo "Then creates a git tag for the new version."
  exit 1
fi

BUMP_TYPE="$1"

if [[ "$BUMP_TYPE" != "major" && "$BUMP_TYPE" != "minor" && "$BUMP_TYPE" != "patch" ]]; then
  echo "Error: argument must be 'major', 'minor', or 'patch'"
  exit 1
fi

# Read current version from package.json
CURRENT_VERSION=$(grep '"version"' package.json | head -1 | sed 's/.*"\([0-9]*\.[0-9]*\.[0-9]*\)".*/\1/')

if [ -z "$CURRENT_VERSION" ]; then
  echo "Error: could not read version from package.json"
  exit 1
fi

IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT_VERSION"

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
esac

NEW_VERSION="${MAJOR}.${MINOR}.${PATCH}"

echo "Bumping version: ${CURRENT_VERSION} -> ${NEW_VERSION} (${BUMP_TYPE})"
echo ""

# Update package.json
sed -i "s/\"version\": \"${CURRENT_VERSION}\"/\"version\": \"${NEW_VERSION}\"/" package.json
echo "  Updated package.json"

# Update Cargo.toml (first occurrence only - the package version)
sed -i "0,/^version = \"${CURRENT_VERSION}\"/s//version = \"${NEW_VERSION}\"/" src-tauri/Cargo.toml
echo "  Updated src-tauri/Cargo.toml"

# Update tauri.conf.json
sed -i "s/\"version\": \"${CURRENT_VERSION}\"/\"version\": \"${NEW_VERSION}\"/" src-tauri/tauri.conf.json
echo "  Updated src-tauri/tauri.conf.json"

# Create git tag
TAG_NAME="v${NEW_VERSION}"
git tag -a "$TAG_NAME" -m "Release ${TAG_NAME}"
echo "  Created git tag: ${TAG_NAME}"

echo ""
echo "Version ${NEW_VERSION} ready."
echo ""
echo "Next steps:"
echo "  git push origin main         # push commits"
echo "  git push origin ${TAG_NAME}  # push tag to trigger release"
