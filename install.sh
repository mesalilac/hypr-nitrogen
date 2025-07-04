#!/usr/bin/env sh

set -e

appimage_path="./src-tauri/target/release/bundle/appimage/hypr-nitrogen_*_amd64.AppImage"

mkdir -pv ~/.local/bin

echo "Bundling hypr-nitrogen"
NO_STRIP=true pnpm tauri build -b appimage

echo "Installing hypr-nitrogen.AppImage"

chmod --verbose +x "${appimage_path}"

cp --verbose "${appimage_path}" ~/.local/bin/hypr-nitrogen
