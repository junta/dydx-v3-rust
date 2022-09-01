Rust client for dYdX (v3 API).

[dYdX API Docs](https://docs.dydx.exchange/)

# Installation

Install [dydx-v3-rust](https://crates.io/crates/dydx-v3-rust) from crates.io. Add the following line to your `Cargo.toml` file's dependencies section:

```rust
[dependencies]
dydx-v3-rust = { git = "https://github.com/junta/dydx-v3-rust" }
tokio = { version = "1.18.2", features = ["full"] }
```

# Usage

## Public API

Sample code to call Get Markets API

```rust
use dydx_v3_rust::{types::*, ClientOptions, DydxClient};

#[tokio::main]
async fn main() {
    let options: ClientOptions = ClientOptions {
        network_id: None,
        api_timeout: None,
        api_key_credentials: None,
        stark_private_key: None,
        eth_private_key: None,
    };
    let client = DydxClient::new("https://api.dydx.exchange", options);
    let response = client
        .public
        .get_markets(Some(DydxMarket::BTC_USD))
        .await
        .unwrap();
    dbg!(response);
}
```

## Private API

Sample code to call Get Accounts API and then Create New Order API(recommend to use pattern match for handling response than unwrap() in your real code.)

```rust
use chrono::{DateTime, Duration, Utc};
use dydx_v3_rust::{types::*, ClientOptions, DydxClient};

#[tokio::main]
async fn main() {
    let api_key = types::ApiKeyCredentials {
        key: "YOUR-API-KEY",
        secret: "YOUR-API-SECRET",
        passphrase: "YOUR-API-PASSPHRASE",
    };
    let options = ClientOptions {
        network_id: Some(1), // mainnet: 1, testnet: 3
        api_timeout: None,
        api_key_credentials: Some(api_key),
        stark_private_key: Some("YOUR-STARK-PRIVATE-KEY"),
        eth_private_key: None, // specify if you call onboarding or ethPrivate functions
    };
    let client = DydxClient::new("https://api.dydx.exchange", options);
    let private = &client.private.unwrap();

    let response = private.get_account("YOUR-ETHEREUM-ADDRESS").await.unwrap();
    dbg!(&response);

    let datetime_now: DateTime<Utc> = Utc::now();
    let expiration = datetime_now + Duration::minutes(3);
    let expiration_unix = expiration.timestamp();

    let position_id = response.account.position_id.as_str();

    let order_params = ApiOrderParams {
        position_id: position_id,
        market: DydxMarket::BTC_USD,
        side: OrderSide::BUY,
        type_field: OrderType::MARKET,
        time_in_force: TimeInForce::FOK,
        post_only: false,
        size: "0.01",
        price: "100000",
        limit_fee: "0.1",
        client_id: None,
        cancel_id: None,
        trigger_price: None,
        trailing_percent: None,
        expiration: expiration_unix,
    };
    let order = private.create_order(order_params).await.unwrap();
    dbg!(order);
}
```

see more examples in tests folder

To call following APIs, you need python shared library to generate signature through [PyO3](https://github.com/PyO3/pyo3) and web3.py.

- Create a new order or Withdraw or Transfer API which requires STARK signature
- Onboarding or EthPrivate(apiKeys) module's API which requires EIP-712-compliant Ethereum signature

Here is sample installation steps via pyenv

```sh
#install pyenv
brew install pyenv

# export PATH
$ echo 'export PYENV_ROOT="$HOME/.pyenv"' >> ~/.bash_profile
$ echo 'export PATH="$PYENV_ROOT/bin:$PATH"' >> ~/.bash_profile
$ echo 'eval "$(pyenv init -)"' >> ~/.bash_profile
$ source ~/.bash_profile

# install python3.9.9 with shared-library
env PYTHON_CONFIGURE_OPTS="--enable-shared" pyenv install 3.9.9
pyenv local 3.9.9
```

Full installation guide: https://github.com/pyenv/pyenv#set-up-your-shell-environment-for-pyenv

Then run pip install.

```sh
pip install -r requirements.txt
```

## Run tests

```sh
cargo test
```
