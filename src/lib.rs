pub mod constants;
pub mod dydx_client;
pub mod error;
pub mod helper;
pub mod modules;
pub mod types;

pub use dydx_client::ClientOptions;
pub use dydx_client::DydxClient;
pub use error::ResponseError;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
