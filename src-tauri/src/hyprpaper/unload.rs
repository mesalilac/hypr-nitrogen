use super::{DispatchErrorKind, Error, Unload, HYPRCTL_CMD, HYPRPAPER_CMD, UNKNOWN_REQUEST_ERROR};
use std::process;

pub fn unload(action: Unload) -> Result<(), Error> {
    let action_string = action.to_string();

    match process::Command::new(HYPRCTL_CMD)
        .args([HYPRPAPER_CMD, "unload", &action_string])
        .output()
    {
        Ok(output) => {
            if let Ok(text) = String::from_utf8(output.stdout.clone()) {
                if text == UNKNOWN_REQUEST_ERROR {
                    log::error!(
                        "Failed to unload wallpaper '{}': unknown request",
                        action_string
                    );
                    return Err(Error::Dispatch(DispatchErrorKind::UnknownRequest));
                }

                if text != "ok\n" {
                    log::error!(
                        "Failed to unload wallpaper '{}': Unexpected: {}",
                        action_string,
                        text
                    );
                    return Err(Error::Dispatch(DispatchErrorKind::UnExpected));
                }
            }
        }
        Err(e) => {
            log::error!("Failed to unload wallpaper '{}': {}", action_string, e);
            return Err(Error::Os(e));
        }
    }

    Ok(())
}
