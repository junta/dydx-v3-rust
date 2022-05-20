Work in progress repo of Rust client for dYdX (v3 API).

[dYdX API Docs](https://docs.dydx.exchange/)

# Installation

Install [dydx-v3-rust](https://crates.io/crates/dydx-v3-rust) from crates.io. Add the following line to your `Cargo.toml` file's dependencies section:

```rust
[dependencies]
dydx-v3-rust = "0.1.0"
tokio = { version = "1.18.2", features = ["full"] }
```

# Usage

## Public API

Sample code to call Get Markets API

```rust
use dydx_v3_rust::*;

#[tokio::main]
async fn main() {
    let options: ClientOptions = ClientOptions {
        network_id: None,
        api_timeout: None,
        api_key_credentials: None,
        stark_private_key: None,
        web3: None,
        eth_private_key: None,
    };
    let client = DydxClient::new("https://api.dydx.exchange", options);
    let response = client
        .public
        .get_markets(Some(types::DydxMarket::BTC_USD))
        .await
        .unwrap();
    dbg!(response);
}
```

## Private API

Sample code to call Get Accounts API and Create New Order API

```rust
use dydx_v3_rust::*;

#[tokio::main]
async fn main() {
    let api_key = types::ApiKeyCredentials {
        key: "YOUR-API-KEY",
        secret: "YOUR-API-SECRET",
        passphrase: "YOUR-API-PASSPHRASE",
    };
    let options = ClientOptions {
        network_id: Some(1),
        api_timeout: None,
        api_key_credentials: Some(api_key),
        stark_private_key: Some("YOUR-STARK-PRIVATE-KEY"),
        web3: None,
        eth_private_key: None,
    };
    let client = DydxClient::new("https://api.dydx.exchange", options);
    let test_address = "0x72Be8d8d7d1d10d0e7f12Df508bB29b33cFFA06B";
    let response = client
        .private
        .unwrap()
        .get_account(test_address)
        .await
        .unwrap();
    dbg!(response);

     let datetime_now: DateTime<Utc> = Utc::now();
                                let expiration = datetime_now + Duration::minutes(3);
                                let expiration_unix = expiration.timestamp();

                                let order_params = ApiOrderParams {
                                        position_id: 62392,
                                        market: DydxMarket::BTC_USD,
                                        side: OrderSide::BUY,
                                        type_field: OrderType::MARKET,
                                        time_in_force: TimeInForce::FOK,
                                        post_only: false,
                                        size: "0.01",
                                        price: "100000",
                                        limit_fee: "0.1",
                                        cancel_id: None,
                                        trigger_price: None,
                                        trailing_percent: None,
                                        expiration: expiration_unix,
                                };
                                let order = DydxClient().private.unwrap().create_order(order_params).await.unwrap();
                                dbg!(order);
}
```

see more examples in tests folder

If you send new orders or withdrawals, you need python shared library to generate DYDX-SIGNATURE through [PyO3](https://github.com/PyO3/pyo3).

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
