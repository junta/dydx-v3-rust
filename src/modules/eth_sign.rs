pub use super::super::types::*;
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::fs;
use std::path::Path;

pub fn sign_private(
    network_id: usize,
    ethereum_address: &str,
    method: &str,
    request_path: &str,
    body: &str,
    expiration_epoch_seconds: &str,
    private_key: &str,
) -> PyResult<String> {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/eth_signing"));
    let py_app = fs::read_to_string(path.join("eth_sign.py"))?;
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast::<PyList>()?;
        syspath.insert(0, &path)?;
        let app: Py<PyAny> = PyModule::from_code(py, &py_app, "", "")?
            .getattr("sign_private")?
            .into();
        app.call1(
            py,
            (
                network_id,
                ethereum_address,
                method,
                request_path,
                body,
                expiration_epoch_seconds,
                private_key,
            ),
        )
    });
    
    Ok(from_python.unwrap().to_string())
}

pub fn sign_onboarding(
    network_id: usize,
    ethereum_address: &str,
    action: &str,
    private_key: &str,
) -> PyResult<String> {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/eth_signing"));
    let py_app = fs::read_to_string(path.join("eth_sign.py"))?;
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast::<PyList>()?;
        syspath.insert(0, &path)?;
        let app: Py<PyAny> = PyModule::from_code(py, &py_app, "", "")?
            .getattr("sign_onboarding")?
            .into();
        app.call1(py, (network_id, ethereum_address, action, private_key))
    });
    
    Ok(from_python.unwrap().to_string())
}

pub fn derive_stark_private_key(signature: String) -> PyResult<String> {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/eth_signing"));
    let py_app = fs::read_to_string(path.join("eth_sign.py"))?;
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast::<PyList>()?;
        syspath.insert(0, &path)?;
        let app: Py<PyAny> = PyModule::from_code(py, &py_app, "", "")?
            .getattr("derive_stark_private_key")?
            .into();
        app.call1(py, (signature,))
    });
    
    Ok(from_python.unwrap().to_string())
}

pub fn derive_secret(hex_value: String) -> PyResult<String> {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/eth_signing"));
    let py_app = fs::read_to_string(path.join("eth_sign.py"))?;
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast::<PyList>()?;
        syspath.insert(0, &path)?;
        let app: Py<PyAny> = PyModule::from_code(py, &py_app, "", "")?
            .getattr("derive_secret")?
            .into();
        app.call1(py, (hex_value,))
    });
    
    Ok(from_python.unwrap().to_string())
}

pub fn derive_passphrase(hex_value: String) -> PyResult<String> {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/eth_signing"));
    let py_app = fs::read_to_string(path.join("eth_sign.py"))?;
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast::<PyList>()?;
        syspath.insert(0, &path)?;
        let app: Py<PyAny> = PyModule::from_code(py, &py_app, "", "")?
            .getattr("derive_passphrase")?
            .into();
        app.call1(py, (hex_value,))
    });
    
    Ok(from_python.unwrap().to_string())
}

pub fn derive_key(hex_value: String) -> PyResult<String> {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/eth_signing"));
    let py_app = fs::read_to_string(path.join("eth_sign.py"))?;
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast::<PyList>()?;
        syspath.insert(0, &path)?;
        let app: Py<PyAny> = PyModule::from_code(py, &py_app, "", "")?
            .getattr("derive_key")?
            .into();
        app.call1(py, (hex_value,))
    });
    
    Ok(from_python.unwrap().to_string())
}
