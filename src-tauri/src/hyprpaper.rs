// TODO: Better Error handling

use serde::Deserialize;
use std::fmt;
use std::process;

const HYPRCTL_CMD: &str = "hyprctl";
const HYPRPAPER_CMD: &str = "hyprpaper";

const UNKNOWN_REQUEST_ERROR: &str = "unknown request\n";
const NO_SUCH_FILE_ERROR: &str = "no such file:";
const WALLPAPER_NOT_PRELOADED: &str = "wallpaper failed (not preloaded)\n";

#[derive(Debug)]
pub enum DispatchErrorKind {
    UnknownRequest,
    NoSuchFile,
    WallpaperNotPreloaded,
    SockConnectionFailed,
}

impl fmt::Display for DispatchErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnknownRequest => write!(f, "Unknown request"),
            Self::NoSuchFile => write!(f, "No such file"),
            Self::WallpaperNotPreloaded => {
                write!(f, "Wallpaper not preloaded")
            }
            Self::SockConnectionFailed => {
                write!(f, "Sock connection failed")
            }
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Os(std::io::Error),
    Dispatch(DispatchErrorKind),
    JsonParsing,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Os(e) => write!(f, "Os error: {}", e),
            Self::Dispatch(e) => write!(f, "Dispatch error: {}", e),
            Self::JsonParsing => write!(f, "Json parsing error"),
        }
    }
}

#[derive(PartialEq)]
pub enum Mode {
    Default,
    Contain,
    Tile,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Default => write!(f, "default"),
            Self::Contain => write!(f, "contain"),
            Self::Tile => write!(f, "tile"),
        }
    }
}

impl Mode {
    pub fn from_string(string: String) -> Self {
        match string.to_lowercase().as_str() {
            "default" => Mode::Default,
            "contain" => Mode::Contain,
            "tile" => Mode::Tile,
            _ => Mode::Default,
        }
    }
}

#[derive(Deserialize, Debug)]
struct Monitor {
    name: String,
}

pub enum Unload {
    All,
    Unused,
    Path(String),
}

impl fmt::Display for Unload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Unused => write!(f, "unused"),
            Self::Path(path) => write!(f, "{}", path),
        }
    }
}

pub fn active_screens() -> Result<Vec<String>, Error> {
    let mut screens: Vec<String> = Vec::new();

    let command = vec!["monitors", "-j"];

    match process::Command::new(HYPRCTL_CMD).args(command).output() {
        Ok(output) => {
            match String::from_utf8(output.stdout.clone()) {
                Ok(text) => {
                    if text == UNKNOWN_REQUEST_ERROR {
                        return Err(Error::Dispatch(DispatchErrorKind::UnknownRequest));
                    }
                }
                Err(_) => {}
            }

            if output.status.success() {
                match serde_json::from_slice::<Vec<Monitor>>(&output.stdout) {
                    Ok(json) => {
                        for item in json {
                            screens.push(item.name);
                        }
                    }
                    Err(_) => return Err(Error::JsonParsing),
                }
            }
        }
        Err(e) => return Err(Error::Os(e)),
    }

    Ok(screens)
}

pub fn unload(wallpaper: Unload) -> Result<bool, Error> {
    let wallpaper_string = wallpaper.to_string();

    let command = vec![HYPRPAPER_CMD, "unload", &wallpaper_string];

    match process::Command::new(HYPRCTL_CMD).args(command).output() {
        Ok(output) => match String::from_utf8(output.stdout.clone()) {
            Ok(text) => {
                if text == UNKNOWN_REQUEST_ERROR {
                    return Err(Error::Dispatch(DispatchErrorKind::UnknownRequest));
                }

                if output.status.success() && text == "Ok\n" {
                    return Ok(true);
                }
            }
            Err(_) => {}
        },
        Err(e) => return Err(Error::Os(e)),
    }

    Ok(true)
}

pub fn preload(wallpaper: String) -> Result<bool, Error> {
    match process::Command::new("sh")
        .args([
            "-c",
            &format!("{} {} preload '{}'", HYPRCTL_CMD, HYPRPAPER_CMD, wallpaper),
        ])
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
            &format!(
                "{} {} wallpaper '{}'",
                HYPRCTL_CMD, HYPRPAPER_CMD, wallpaper_command_value
            ),
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
