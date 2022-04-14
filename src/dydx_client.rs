pub use super::structs;
use crate::modules::private::Private;
use crate::modules::public::Public;

// #[derive(Debug, Copy, Clone)]
pub struct ClientOptions<'a> {
    pub network_id: Option<usize>,
    pub api_timeout: Option<usize>,
    pub api_key_credentials: Option<structs::ApiKeyCredentials<'a>>,
    pub stark_private_key: Option<&'a str>,
}

#[readonly::make]
pub struct DydxClient<'a> {
    #[readonly]
    pub host: &'a str,
    #[readonly]
    pub network_id: usize,
    #[readonly]
    pub api_timeout: Option<usize>,
    pub api_key_credentials: Option<structs::ApiKeyCredentials<'a>>,
    pub public: Public<'a>,
    pub private: Option<Private<'a>>,
    // pub stark_private_key: Option<&'a str>,
}

impl DydxClient<'_> {
    pub fn new<'a>(host: &'a str, _options: ClientOptions<'a>) -> DydxClient<'a> {
        let network_id = _options.network_id.unwrap_or(1);
        DydxClient {
            host,
            network_id,
            api_timeout: None,
            api_key_credentials: _options.api_key_credentials.clone(),
            public: Public::new(host),
            private: match _options.api_key_credentials {
                Some(v) => Some(Private::new(
                    host,
                    network_id,
                    v,
                    _options.stark_private_key,
                )),
                None => None,
            },
        }
    }
}
