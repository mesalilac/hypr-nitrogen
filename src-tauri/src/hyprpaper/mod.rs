// TODO: Better Error handling

mod active_screens;
mod preload;
mod set_wallpaper;
mod unload;

pub use active_screens::active_screens;
pub use set_wallpaper::set_wallpaper;
pub use unload::unload;

use std::fmt;

pub const HYPRCTL_CMD: &str = "hyprctl";
pub const HYPRPAPER_CMD: &str = "hyprpaper";

pub const UNKNOWN_REQUEST_ERROR: &str = "unknown request\n";
pub const NO_SUCH_FILE_ERROR: &str = "no such file:";
pub const WALLPAPER_NOT_PRELOADED: &str = "wallpaper failed (not preloaded)\n";

#[derive(Debug)]
pub enum DispatchErrorKind {
    UnknownRequest,
    NoSuchFile,
    WallpaperNotPreloaded,
    SockConnectionFailed,
    UnExpected,
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
            Self::UnExpected => {
                write!(f, "An unexpected error occurred")
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
            Self::Os(e) => write!(f, "Os error: {e}"),
            Self::Dispatch(e) => write!(f, "Dispatch error: {e}"),
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

#[allow(dead_code)]
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
            Self::Path(path) => write!(f, "{path}"),
        }
    }
}
