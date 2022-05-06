use pyo3::prelude::*;
use pyo3::types::PyList;
use std::fs;
use std::path::Path;

pub fn sign_order() -> PyResult<String> {
    let path = Path::new("./src/stark");
    let py_app = fs::read_to_string(path.join("stark_sign.py"))?;
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast::<PyList>()?;
        syspath.insert(0, &path)?;
        let app: Py<PyAny> = PyModule::from_code(py, &py_app, "", "")?
            .getattr("sign_order")?
            .into();
        app.call1(py, (3, "ETH-USD", "BUY"))
    });
    // println!("py: {}", from_python?);
    Ok(from_python.unwrap().to_string())
}
