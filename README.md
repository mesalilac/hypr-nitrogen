## Preview

![preview image](./preview.png)

## Dependencies

- [hyprctl](https://github.com/hyprwm/Hyprland)
- [imagemagick - magick](https://imagemagick.org/script/magick.php)

## Build

See: https://github.com/linuxdeploy/linuxdeploy/issues/272

```
NO_STRIP=true cargo tauri build -b appimage
```

Copy the app file from `src-tauri/target/release/bundle/appimage` to ~/.local/bin
