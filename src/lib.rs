pub mod dydx_client;
pub mod error;
pub mod helper;
pub mod modules;
pub mod private;
pub mod public;
pub mod structs;

pub use dydx_client::ClientOptions;
pub use dydx_client::DydxClient;
pub use error::ResponseError;
pub use private::Private;
pub use public::Public;

// pub type Result<T> = std::result::Result<T, error::Error>;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
