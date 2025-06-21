#!/usr/bin/env sh

set -e

mkdir -pv ~/.local/bin

echo "Bundling hypr-nitrogen"
NO_STRIP=true pnpm tauri build

echo "Installing hypr-nitrogen.AppImage"
cp -v ./src-tauri/target/release/bundle/appimage/hypr-nitrogen_0.1.0_amd64.AppImage ~/.local/bin/hypr-nitrogen.AppImage
