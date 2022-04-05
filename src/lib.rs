pub mod api;
pub mod client;
pub mod error;
pub mod modules;
pub mod structs;

use api::Api;
pub use client::Client;
pub use client::Response;
pub use error::Error;

pub type Result<T> = std::result::Result<T, error::Error>;
