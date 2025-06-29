use super::{DispatchErrorKind, Error, Unload, HYPRCTL_CMD, HYPRPAPER_CMD, UNKNOWN_REQUEST_ERROR};
use std::process;

pub fn unload(wallpaper: Unload) -> Result<(), Error> {
    let wallpaper_string = wallpaper.to_string();

    match process::Command::new(HYPRCTL_CMD)
        .args([HYPRPAPER_CMD, "unload", &wallpaper_string])
        .output()
    {
        Ok(output) => {
            if let Ok(text) = String::from_utf8(output.stdout.clone()) {
                if text == UNKNOWN_REQUEST_ERROR {
                    return Err(Error::Dispatch(DispatchErrorKind::UnknownRequest));
                }

                if text != "ok\n" {
                    return Err(Error::Dispatch(DispatchErrorKind::UnExpected));
                }
            }
        }
        Err(e) => return Err(Error::Os(e)),
    }

    Ok(())
}
