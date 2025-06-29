use super::{
    preload, DispatchErrorKind, Error, Mode, HYPRCTL_CMD, HYPRPAPER_CMD, NO_SUCH_FILE_ERROR,
    UNKNOWN_REQUEST_ERROR, WALLPAPER_NOT_PRELOADED,
};
use std::process;

pub fn set_wallpaper(screen: String, wallpaper: String, mode: &Mode) -> Result<bool, Error> {
    let mode_string = mode.to_string();

    if !std::path::Path::new(&wallpaper).exists() {
        println!("Wallpaper not found, '{}'", wallpaper);
        return Err(Error::Dispatch(DispatchErrorKind::NoSuchFile));
    }

    preload(wallpaper.clone())?;

    let mut wallpaper_command_value = String::new();

    // NOTE: screen is disable because im having a problem with setting a wallpaper for all screens
    // when a wallpaper is set for a single screen you can't change it
    // using `hyprctl hyprpaper ",<path>"` you need to target the screen
    //
    if screen != "all" {
        // wallpaper_command_value.push_str(&screen);
    }
    wallpaper_command_value.push(',');
    if mode != &Mode::Default {
        wallpaper_command_value.push_str(&mode_string);
        wallpaper_command_value.push(':');
    }
    wallpaper_command_value.push_str(&wallpaper);

    let cmd = process::Command::new("sh")
        .args([
            "-c",
            HYPRCTL_CMD,
            HYPRPAPER_CMD,
            "wallpaper",
            &wallpaper_command_value,
        ])
        .output();

    match cmd {
        Ok(output) => {
            if let Ok(text) = String::from_utf8(output.stdout.clone()) {
                if text == UNKNOWN_REQUEST_ERROR {
                    return Err(Error::Dispatch(DispatchErrorKind::UnknownRequest));
                } else if text.starts_with(NO_SUCH_FILE_ERROR) {
                    return Err(Error::Dispatch(DispatchErrorKind::NoSuchFile));
                } else if text == WALLPAPER_NOT_PRELOADED {
                    return Err(Error::Dispatch(DispatchErrorKind::WallpaperNotPreloaded));
                } else if text.contains("no such file:") {
                    return Err(Error::Dispatch(DispatchErrorKind::NoSuchFile));
                } else if text.starts_with("Couldn't connect to") {
                    return Err(Error::Dispatch(DispatchErrorKind::SockConnectionFailed));
                }

                if output.status.success() && text == "Ok\n" {
                    return Ok(true);
                }
            }
        }
        Err(e) => return Err(Error::Os(e)),
    }

    Ok(true)
}
