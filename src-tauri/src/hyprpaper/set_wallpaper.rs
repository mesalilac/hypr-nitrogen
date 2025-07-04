use super::{
    active_screens, unload, DispatchErrorKind, Error, Mode, Unload, HYPRCTL_CMD, HYPRPAPER_CMD,
    NO_SUCH_FILE_ERROR, UNKNOWN_REQUEST_ERROR, WALLPAPER_NOT_PRELOADED,
};
use std::process;

pub fn set_wallpaper(screen: String, wallpaper: String, mode: &Mode) -> Result<(), Error> {
    let mode_string = mode.to_string();

    if !std::path::Path::new(&wallpaper).exists() {
        log::error!("Wallpaper not found, '{}'", wallpaper);
        return Err(Error::Dispatch(DispatchErrorKind::NoSuchFile));
    }

    unload(Unload::All)?;

    let mut screens: Vec<String> = Vec::new();

    if screen == "all" {
        screens = active_screens()?;
    } else {
        screens.push(screen);
    }

    for target_screen in screens {
        let mut wallpaper_command_value = String::new();

        wallpaper_command_value.push_str(&target_screen);
        wallpaper_command_value.push(',');
        if mode != &Mode::Default {
            wallpaper_command_value.push_str(&mode_string);
            wallpaper_command_value.push(':');
        }
        wallpaper_command_value.push_str(&wallpaper);

        let cmd = process::Command::new(HYPRCTL_CMD)
            .args([HYPRPAPER_CMD, "reload", &wallpaper_command_value])
            .output();

        match cmd {
            Ok(output) => {
                if let Ok(text) = String::from_utf8(output.stdout.clone()) {
                    if text == UNKNOWN_REQUEST_ERROR {
                        log::error!(
                            "Failed to set wallpaper('{}', '{}', '{}'): unknown request",
                            target_screen,
                            wallpaper,
                            mode_string
                        );
                        return Err(Error::Dispatch(DispatchErrorKind::UnknownRequest));
                    } else if text.starts_with(NO_SUCH_FILE_ERROR) {
                        log::error!(
                            "Failed to set wallpaper('{}', '{}', '{}'): no such file",
                            target_screen,
                            wallpaper,
                            mode_string
                        );
                        return Err(Error::Dispatch(DispatchErrorKind::NoSuchFile));
                    } else if text == WALLPAPER_NOT_PRELOADED {
                        log::error!(
                            "Failed to set wallpaper('{}', '{}', '{}'): wallpaper not preloaded",
                            target_screen,
                            wallpaper,
                            mode_string
                        );
                        return Err(Error::Dispatch(DispatchErrorKind::WallpaperNotPreloaded));
                    } else if text.contains("no such file:") {
                        log::error!(
                            "Failed to set wallpaper('{}', '{}', '{}'): no such file",
                            target_screen,
                            wallpaper,
                            mode_string
                        );
                        return Err(Error::Dispatch(DispatchErrorKind::NoSuchFile));
                    } else if text.starts_with("Couldn't connect to") {
                        log::error!(
                            "Failed to set wallpaper('{}', '{}', '{}'): sock connection failed",
                            target_screen,
                            wallpaper,
                            mode_string
                        );
                        return Err(Error::Dispatch(DispatchErrorKind::SockConnectionFailed));
                    }

                    if text != "ok\n" {
                        log::error!(
                            "Failed to set wallpaper('{}', '{}', '{}'): Unexpected: {}",
                            target_screen,
                            wallpaper,
                            mode_string,
                            text
                        );
                        return Err(Error::Dispatch(DispatchErrorKind::UnExpected));
                    }
                }
            }
            Err(e) => {
                log::error!(
                    "Failed to set wallpaper('{}', '{}', '{}'): {}",
                    target_screen,
                    wallpaper,
                    mode_string,
                    e
                );
                return Err(Error::Os(e));
            }
        }

        // hyprpaper will crash if you don't wait after setting the first wallpaper
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
