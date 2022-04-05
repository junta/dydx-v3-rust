pub use super::structs;
use super::Public;

pub struct ClientOptions {
    pub networkId: Option<usize>,
    pub apiTimeout: Option<usize>,
}

pub struct DydxClient<'a> {
    pub host: &'a str,
    pub networkId: Option<usize>,
    pub apiTimeout: Option<usize>,
    pub public: Public<'a>,
}

impl DydxClient<'_> {
    pub fn new(host: &str, _options: Option<ClientOptions>) -> DydxClient {
        DydxClient {
            host,
            networkId: None,
            apiTimeout: None,
            public: Public::new(host),
        }
    }
}
