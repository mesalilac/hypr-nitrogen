#!/usr/bin/env sh

set -e

echo "Removeing hypr-nitrogen.AppImage"
rm --verbose ~/.local/bin/hypr-nitrogen

echo "Removing app data directory"
rm -rfv ~/.local/share/hypr-nitrogen/

echo "Removing cache directory"
rm -rfv ~/.cache/hypr-nitrogen/
