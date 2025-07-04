#![allow(clippy::uninlined_format_args)]

pub mod config;
pub mod error;
pub mod utils;

pub use config::Config;
pub use error::{ClientError, Result};
pub use utils::*;

pub mod deploy;
pub mod retrive_funds;
pub mod verify;
