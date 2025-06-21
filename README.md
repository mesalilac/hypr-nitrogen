# Tauri + Solid + Typescript

This template should help get you started developing with Tauri, Solid and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Build

See: https://github.com/linuxdeploy/linuxdeploy/issues/272

```
NO_STRIP=true pnpm tauri build
```

Copy the app file from `src-tauri/target/release/bundle/appimage` to ~/.local/bin
