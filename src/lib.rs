pub mod dydx_client;
pub mod error;
pub mod eth_signing;
pub mod helper;
pub mod modules;
pub mod structs;

pub use dydx_client::ClientOptions;
pub use dydx_client::DydxClient;
pub use error::ResponseError;
// pub use eth_signing::eth_private_action::SignEthPrivateAction;
pub use eth_signing::onboarding_action::OnboardingAction;

// pub type Result<T> = std::result::Result<T, error::Error>;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
