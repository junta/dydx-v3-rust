Work in progress repo of Rust client for dYdX (v3 API).

[dYdX API Docs](https://docs.dydx.exchange/)

# Installation

Install [dydx-v3-rust](https://crates.io/crates/dydx-v3-rust) from crates.io. Add the following line to your `Cargo.toml` file's dependencies section:

```rust
[dependencies]
dydx-v3-rust = "0.1.0"
```

# Usage

## Public API

```rust
use dydx_v3_rust::DydxClient;
 fn DydxClient() -> DydxClient<'static> {

    let options: ClientOptions = ClientOptions {
            network_id: None,
                                api_timeout: None,
                                api_key_credentials: None,
                                stark_private_key: None,
                                web3: None,
                                eth_private_key: None
                        };
                        DydxClient::new("https://api.dydx.exchange", options)
                    }
        let response = DydxClient().public.get_markets(Some(DydxMarket::BTC_USD)).await.unwrap();

```

## Private API

```rust
use dydx_v3_rust::DydxClient;
```

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
