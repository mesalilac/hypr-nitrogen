use super::{DispatchErrorKind, Error, Unload, HYPRCTL_CMD, HYPRPAPER_CMD, UNKNOWN_REQUEST_ERROR};
use std::process;

pub fn unload(wallpaper: Unload) -> Result<bool, Error> {
    let wallpaper_string = wallpaper.to_string();

    match process::Command::new("sh")
        .args([
            "-c",
            HYPRCTL_CMD,
            HYPRPAPER_CMD,
            "unload",
            &wallpaper_string,
        ])
        .output()
    {
        Ok(output) => {
            if let Ok(text) = String::from_utf8(output.stdout.clone()) {
                if text == UNKNOWN_REQUEST_ERROR {
                    return Err(Error::Dispatch(DispatchErrorKind::UnknownRequest));
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
