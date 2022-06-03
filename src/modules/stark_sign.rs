use pyo3::prelude::*;
use pyo3::types::PyList;
use std::fs;
use std::path::Path;

pub fn sign_order(
    network_id: usize,
    market: &str,
    side: &str,
    position_id: &str,
    human_size: &str,
    human_price: &str,
    limit_fee: &str,
    client_id: &str,
    expiration_epoch_seconds: i64,
    private_key: &str,
) -> PyResult<String> {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/stark"));
    let py_app = fs::read_to_string(path.join("stark_sign.py"))?;
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast::<PyList>()?;
        syspath.insert(0, &path)?;
        let app: Py<PyAny> = PyModule::from_code(py, &py_app, "", "")?
            .getattr("sign_order")?
            .into();
        app.call1(
            py,
            (
                network_id,
                market,
                side,
                position_id,
                human_size,
                human_price,
                limit_fee,
                client_id,
                expiration_epoch_seconds,
                private_key,
            ),
        )
    });
    
    Ok(from_python.unwrap().to_string())
}

pub fn sign_withdraw(
    network_id: usize,
    position_id: &str,
    amount: &str,
    client_id: &str,
    expiration_epoch_seconds: i64,
    private_key: &str,
) -> PyResult<String> {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/stark"));
    let py_app = fs::read_to_string(path.join("stark_sign.py"))?;
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast::<PyList>()?;
        syspath.insert(0, &path)?;
        let app: Py<PyAny> = PyModule::from_code(py, &py_app, "", "")?
            .getattr("sign_withdraw")?
            .into();
        app.call1(
            py,
            (
                network_id,
                position_id,
                amount,
                client_id,
                expiration_epoch_seconds,
                private_key,
            ),
        )
    });

    Ok(from_python.unwrap().to_string())
}

pub fn sign_fast_withdraw(
    network_id: usize,
    sender_position_id: &str,
    receiver_position_id: &str,
    receiver_public_key: &str,
    fact_registry_address: &str,
    recipient: &str,
    token_decimals: u8,
    human_amount: &str,
    token_address: &str,
    // salt: usize,
    client_id: &str,
    expiration_epoch_seconds: i64,
    private_key: &str,
) -> PyResult<String> {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/stark"));
    let py_app = fs::read_to_string(path.join("stark_sign.py"))?;
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast::<PyList>()?;
        syspath.insert(0, &path)?;
        let app: Py<PyAny> = PyModule::from_code(py, &py_app, "", "")?
            .getattr("sign_fast_withdraw")?
            .into();
        app.call1(
            py,
            (
                network_id,
                sender_position_id,
                receiver_position_id,
                receiver_public_key,
                fact_registry_address,
                recipient,
                token_decimals,
                human_amount,
                token_address,
                // salt,
                client_id,
                expiration_epoch_seconds,
                private_key,
            ),
        )
    });
    
    Ok(from_python.unwrap().to_string())
}

pub fn sign_transfer(
    network_id: usize,
    sender_position_id: &str,
    receiver_position_id: &str,
    receiver_public_key: &str,
    human_amount: &str,
    client_id: &str,
    expiration_epoch_seconds: i64,
    private_key: &str,
) -> PyResult<String> {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/stark"));
    let py_app = fs::read_to_string(path.join("stark_sign.py"))?;
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast::<PyList>()?;
        syspath.insert(0, &path)?;
        let app: Py<PyAny> = PyModule::from_code(py, &py_app, "", "")?
            .getattr("sign_transfer")?
            .into();
        app.call1(
            py,
            (
                network_id,
                sender_position_id,
                receiver_position_id,
                receiver_public_key,
                human_amount,
                client_id,
                expiration_epoch_seconds,
                private_key,
            ),
        )
    });
    
    Ok(from_python.unwrap().to_string())
}
