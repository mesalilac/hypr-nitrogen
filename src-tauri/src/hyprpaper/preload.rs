use super::{
    DispatchErrorKind, Error, HYPRCTL_CMD, HYPRPAPER_CMD, NO_SUCH_FILE_ERROR, UNKNOWN_REQUEST_ERROR,
};
use std::process;

pub fn preload(wallpaper: String) -> Result<(), Error> {
    match process::Command::new(HYPRCTL_CMD)
        .args([HYPRPAPER_CMD, "preload", &wallpaper])
        .output()
    {
        Ok(output) => {
            if let Ok(text) = String::from_utf8(output.stdout.clone()) {
                if text == UNKNOWN_REQUEST_ERROR {
                    log::error!(
                        "Failed to preload wallpaper '{wallpaper}': unknown request"
                    );
                    return Err(Error::Dispatch(DispatchErrorKind::UnknownRequest));
                } else if text.starts_with(NO_SUCH_FILE_ERROR) {
                    log::error!("Failed to preload wallpaper '{wallpaper}': no such file");
                    return Err(Error::Dispatch(DispatchErrorKind::NoSuchFile));
                }

                if text != "ok\n" {
                    log::error!(
                        "Failed to preload wallpaper '{wallpaper}': Unexpected: {text}"
                    );
                    return Err(Error::Dispatch(DispatchErrorKind::UnExpected));
                }
            }
        }
        Err(e) => {
            log::error!("Failed to preload wallpaper '{wallpaper}': {e}");
            return Err(Error::Os(e));
        }
    }

    Ok(())
}
