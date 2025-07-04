use serde::Deserialize;
use std::process;

use super::{DispatchErrorKind, Error, HYPRCTL_CMD, UNKNOWN_REQUEST_ERROR};

#[derive(Deserialize, Debug)]
struct Monitor {
    name: String,
}

pub fn active_screens() -> Result<Vec<String>, Error> {
    let mut screens: Vec<String> = Vec::new();

    match process::Command::new(HYPRCTL_CMD)
        .args(["monitors", "-j"])
        .output()
    {
        Ok(output) => {
            if let Ok(text) = String::from_utf8(output.stdout.clone()) {
                if text == UNKNOWN_REQUEST_ERROR {
                    log::error!("Failed to get active screens: unknown request");
                    return Err(Error::Dispatch(DispatchErrorKind::UnknownRequest));
                }
            }

            if output.status.success() {
                match serde_json::from_slice::<Vec<Monitor>>(&output.stdout) {
                    Ok(json) => {
                        for item in json {
                            screens.push(item.name);
                        }
                    }
                    Err(_) => {
                        log::error!("Failed to get active screens, json parsing failed");
                        return Err(Error::JsonParsing);
                    }
                }
            }
        }
        Err(e) => {
            log::error!("Failed to get active screens: {}", e);
            return Err(Error::Os(e));
        }
    }

    Ok(screens)
}
