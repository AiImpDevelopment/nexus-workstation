#!/usr/bin/env bash
# auto-push.sh - Auto-commit and push changes in the Nexus project
# Usage: ./scripts/auto-push.sh [--tag]
set -euo pipefail

NEXUS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$NEXUS_DIR"

TAG_FLAG=false
for arg in "$@"; do
  case "$arg" in
    --tag) TAG_FLAG=true ;;
    -h|--help)
      echo "Usage: $0 [--tag]"
      echo ""
      echo "Checks for changes, auto-increments patch version,"
      echo "creates a conventional commit, and pushes to origin main."
      echo ""
      echo "Options:"
      echo "  --tag    Create a version tag and trigger the release workflow"
      echo "  -h       Show this help"
      exit 0
      ;;
    *)
      echo "Unknown argument: $arg"
      exit 1
      ;;
  esac
done

# Check for uncommitted changes
if git diff --quiet && git diff --cached --quiet && [ -z "$(git ls-files --others --exclude-standard)" ]; then
  echo "No changes detected. Nothing to push."
  exit 0
fi

# Detect what changed for commit message
CHANGED_FILES=$(git diff --name-only HEAD 2>/dev/null || true)
STAGED_FILES=$(git diff --cached --name-only 2>/dev/null || true)
UNTRACKED_FILES=$(git ls-files --others --exclude-standard 2>/dev/null || true)
ALL_CHANGED="${CHANGED_FILES}${STAGED_FILES}${UNTRACKED_FILES}"

# Determine commit type from changed files
COMMIT_TYPE="chore"
COMMIT_SCOPE=""
COMMIT_DESC="update project files"

if echo "$ALL_CHANGED" | grep -q "^src/"; then
  COMMIT_TYPE="feat"
  COMMIT_SCOPE="ui"
  COMMIT_DESC="update frontend components"
fi

if echo "$ALL_CHANGED" | grep -q "^src-tauri/"; then
  COMMIT_TYPE="feat"
  COMMIT_SCOPE="core"
  COMMIT_DESC="update Tauri backend"
fi

if echo "$ALL_CHANGED" | grep -q "\.github/"; then
  COMMIT_TYPE="ci"
  COMMIT_SCOPE="workflows"
  COMMIT_DESC="update CI/CD pipelines"
fi

if echo "$ALL_CHANGED" | grep -q "scripts/"; then
  COMMIT_TYPE="chore"
  COMMIT_SCOPE="scripts"
  COMMIT_DESC="update build scripts"
fi

if echo "$ALL_CHANGED" | grep -q "package.json\|pnpm-lock.yaml\|Cargo.toml\|Cargo.lock"; then
  COMMIT_TYPE="chore"
  COMMIT_SCOPE="deps"
  COMMIT_DESC="update dependencies"
fi

# Build scope string
if [ -n "$COMMIT_SCOPE" ]; then
  COMMIT_PREFIX="${COMMIT_TYPE}(${COMMIT_SCOPE})"
else
  COMMIT_PREFIX="${COMMIT_TYPE}"
fi

# Auto-increment patch version
CURRENT_VERSION=$(grep '"version"' package.json | head -1 | sed 's/.*"\([0-9]*\.[0-9]*\.[0-9]*\)".*/\1/')
IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT_VERSION"
NEW_PATCH=$((PATCH + 1))
NEW_VERSION="${MAJOR}.${MINOR}.${NEW_PATCH}"

echo "Version bump: ${CURRENT_VERSION} -> ${NEW_VERSION}"

# Update version in package.json
sed -i "s/\"version\": \"${CURRENT_VERSION}\"/\"version\": \"${NEW_VERSION}\"/" package.json

# Update version in Cargo.toml (only the package version line)
sed -i "0,/^version = \"${CURRENT_VERSION}\"/s//version = \"${NEW_VERSION}\"/" src-tauri/Cargo.toml

# Update version in tauri.conf.json
sed -i "s/\"version\": \"${CURRENT_VERSION}\"/\"version\": \"${NEW_VERSION}\"/" src-tauri/tauri.conf.json

# Stage all changes
git add -A

# Create commit
COMMIT_MSG="${COMMIT_PREFIX}: ${COMMIT_DESC} (v${NEW_VERSION})"
echo "Committing: ${COMMIT_MSG}"
git commit -m "$COMMIT_MSG"

# Push to origin
echo "Pushing to origin main..."
git push origin main

# Optionally create tag to trigger release
if [ "$TAG_FLAG" = true ]; then
  TAG_NAME="v${NEW_VERSION}"
  echo "Creating tag: ${TAG_NAME}"
  git tag -a "$TAG_NAME" -m "Release ${TAG_NAME}"
  git push origin "$TAG_NAME"
  echo "Tag ${TAG_NAME} pushed. Release workflow will be triggered."
fi

echo ""
echo "Done! Version ${NEW_VERSION} pushed to origin main."
