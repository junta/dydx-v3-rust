/// Error for API calls from [`Client`](super::Client).
#[derive(Debug)]
pub enum Error {
    AuthenticationError,
    ApiConnectionError,
    InvalidRequestError,
    NotFoundError,
    Reqwest(reqwest::Error),
    Json(serde_json::Error),
    NoStarkKeyError,
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Json(e)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Reqwest(ref e) => Some(e),
            Error::Json(ref e) => Some(e),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::AuthenticationError => {
                write!(f, "Wrong api key.")
            }
            Error::ApiConnectionError => {
                write!(f, "Fail to connect to API servers.")
            }
            Error::InvalidRequestError => {
                write!(
                    f,
                    "Something wrong on your end (client side errors), e.g., missing required parameters."
                )
            }

            Error::NotFoundError => {
                write!(f, "Endpoint not exist")
            }

            Error::Reqwest(_) | Error::Json(_) => {
                write!(f, "{}", self)
            }

            Error::NoStarkKeyError => {
                write!(
                    f,
                    "Order is not signed and client was not initialized with starkPrivateKey"
                )
            }
        }
    }
}
