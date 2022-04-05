pub mod dydx_client;
pub mod error;
pub mod modules;
pub mod public;
pub mod structs;

pub use dydx_client::DydxClient;
pub use public::Public;

pub type Result<T> = std::result::Result<T, error::Error>;
