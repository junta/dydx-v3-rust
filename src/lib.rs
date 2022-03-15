mod api;
mod client;
mod error;
pub mod public;

use api::Api;
pub use client::Client;
pub use client::Response;
pub use error::Error;

pub type Result<T> = std::result::Result<T, error::Error>;
