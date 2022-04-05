pub use super::structs;
use super::Private;
use super::Public;

// #[derive(Debug, Copy, Clone)]
pub struct ClientOptions<'a> {
    pub network_id: Option<usize>,
    pub api_timeout: Option<usize>,
    pub api_key_credentials: Option<structs::ApiKeyCredentials<'a>>,
}

pub struct DydxClient<'a> {
    pub host: &'a str,
    pub network_id: usize,
    pub api_timeout: Option<usize>,
    pub api_key_credentials: Option<structs::ApiKeyCredentials<'a>>,
    pub public: Public<'a>,
    // pub private: Option<Private<'a>>,
    pub private: Private<'a>,
}

impl DydxClient<'_> {
    pub fn new<'a>(host: &'a str, _options: Option<ClientOptions<'a>>) -> DydxClient<'a> {
        let api_key = structs::ApiKeyCredentials {
            key: "ed85a071-c6b4-b4f1-c965-efb238d16c5e",
            secret: "1iDz27dyq4RspTkP-rfTcFN6ouxTgHmTT_sKJogU",
            passphrase: "CfbXaq6O-Yd3jKOqh10i",
        };

        let network_id = _options.unwrap().network_id.unwrap_or(1);
        // let private = Private::new(host, &_options.unwrap().network_id.unwrap(), api_key);

        DydxClient {
            host,
            network_id: network_id,
            // network_id: match &_options {
            //     Some(v) => v.network_id.unwrap_or(1),
            //     None => 1,
            // },
            api_timeout: None,
            // api_key_credentials: match _options {
            //     Some(v) => Some(v.api_key_credentials).unwrap_or(None),
            //     None => None,
            // },
            api_key_credentials: None,
            public: Public::new(host),
            private: Private::new(host, network_id, api_key),
        }
    }
}
