mod get;
mod remove;
mod set;
mod util;

pub use get::*;
pub use remove::*;
pub use set::*;
pub use util::*;

use serde::Serialize;

#[derive(Serialize)]
pub struct IpcError {
    pub message: String,
    pub details: Option<String>,
}

#[derive(Serialize)]
pub struct Response<T> {
    pub is_ok: bool,
    pub data: Option<T>,
    pub error: Option<IpcError>,
}

impl<T> Response<T> {
    pub fn ok(data: T) -> Self {
        Self {
            is_ok: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String, details: Option<String>) -> Self {
        Self {
            is_ok: false,
            data: None,
            error: Some(IpcError { message, details }),
        }
    }
}
