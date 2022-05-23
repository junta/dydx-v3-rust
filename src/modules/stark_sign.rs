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
    // expiration_epoch_seconds: &str,
    expiration_epoch_seconds: i64,
    private_key: &str,
    path: &str,
) -> PyResult<String> {
    // let path = Path::new("./src/stark");
    let path = Path::new(path);
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
    // println!("py: {}", from_python?);
    Ok(from_python.unwrap().to_string())
}

pub fn sign_withdraw(
    network_id: usize,
    position_id: &str,
    amount: &str,
    client_id: &str,
    expiration_epoch_seconds: i64,
    private_key: &str,
    path: &str,
) -> PyResult<String> {
    // let path = Path::new("./src/stark");
    let path = Path::new(path);
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
    // println!("py: {}", from_python?);
    Ok(from_python.unwrap().to_string())
}
