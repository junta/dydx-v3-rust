use pyo3::prelude::*;
use pyo3::types::PyList;
use std::fs;
use std::path::Path;

pub fn sign(
    network_id: usize,
    ethereum_address: &str,
    method: &str,
    request_path: &str,
    body: &str,
    expiration_epoch_seconds: &str,
    private_key: &str,
    // path: &str,
) -> PyResult<String> {
    let path = Path::new("./src/eth_signing");
    // let path = Path::new(path);
    let py_app = fs::read_to_string(path.join("eth_sign.py"))?;
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast::<PyList>()?;
        syspath.insert(0, &path)?;
        let app: Py<PyAny> = PyModule::from_code(py, &py_app, "", "")?
            .getattr("sign")?
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
    // println!("py: {}", from_python?);
    Ok(from_python.unwrap().to_string())
}
