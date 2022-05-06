use self::structs::ApiKeyCredentials;

pub use super::structs;
use crate::modules::eth_private::EthPrivate;
use crate::modules::onboarding::Onboarding;
use crate::modules::private::Private;
use crate::modules::public::Public;
use web3::transports::Http;

#[derive(Debug)]
pub struct ClientOptions<'a> {
    pub network_id: Option<usize>,
    pub api_timeout: Option<usize>,
    pub api_key_credentials: Option<structs::ApiKeyCredentials>,
    pub stark_private_key: Option<&'a str>,
    pub web3: Option<web3::Web3<Http>>,
    pub eth_private_key: Option<String>,
}

#[readonly::make]
pub struct DydxClient<'a> {
    #[readonly]
    pub host: &'a str,
    #[readonly]
    pub network_id: usize,
    #[readonly]
    pub api_timeout: Option<usize>,
    // pub api_key_credentials: Option<structs::ApiKeyCredentials<'a>>,
    pub public: Public<'a>,
    pub private: Option<Private<'a>>,
    pub eth_private: Option<EthPrivate<'a>>,
    pub onboarding: Option<Onboarding<'a>>,
    // pub stark_private_key: Option<&'a str>,
}

impl DydxClient<'_> {
    pub fn new<'a>(host: &'a str, _options: ClientOptions<'a>) -> DydxClient<'a> {
        let network_id = _options.network_id.unwrap_or(1);
        DydxClient {
            host,
            network_id,
            api_timeout: None,
            // api_key_credentials: _options.api_key_credentials.clone(),
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
            eth_private: match _options.web3 {
                Some(v) => Some(EthPrivate::new(
                    host,
                    network_id,
                    v,
                    _options.eth_private_key.as_ref().unwrap(),
                )),
                None => None,
            },
            onboarding: match _options.eth_private_key {
                Some(r) => Some(Onboarding::new(host, network_id)),
                None => None,
            },
        }
    }
}
