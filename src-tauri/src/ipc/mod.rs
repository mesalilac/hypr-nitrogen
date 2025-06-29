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
pub struct Response<T> {
    pub data: T,
}

impl<T> Response<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}
