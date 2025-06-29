use super::{
    DispatchErrorKind, Error, HYPRCTL_CMD, HYPRPAPER_CMD, NO_SUCH_FILE_ERROR, UNKNOWN_REQUEST_ERROR,
};
use std::process;

pub fn preload(wallpaper: String) -> Result<bool, Error> {
    match process::Command::new("sh")
        .args(["-c", HYPRCTL_CMD, HYPRPAPER_CMD, "preload", &wallpaper])
        .output()
    {
        Ok(output) => {
            if let Ok(text) = String::from_utf8(output.stdout.clone()) {
                if text == UNKNOWN_REQUEST_ERROR {
                    return Err(Error::Dispatch(DispatchErrorKind::UnknownRequest));
                } else if text.starts_with(NO_SUCH_FILE_ERROR) {
                    return Err(Error::Dispatch(DispatchErrorKind::NoSuchFile));
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
