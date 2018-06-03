#![feature(proc_macro)]
#![feature(proc_macro_path_invoc)]

extern crate pyo3;
extern crate glob;


use pyo3::prelude::*;
use glob::glob;
use std::result::Result;


pub fn register_constants(py: Python, m: &PyModule) -> PyResult<()> {
    // FIXME:
    // m.add("magic_check", None);         // re.compile('([*?[])')
    // m.add("magic_check_bytes", None);   // re.compile(b'([*?[])')
    Ok(())
}

#[py::modinit(_rsglob)]
fn init(py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "escape")]
    pub fn escape(py: Python) -> PyResult<PyObject> { Ok(py.None()) }
    #[pyfn(m, "glob0")]
    pub fn glob0(py: Python) -> PyResult<PyObject> { Ok(py.None()) }

    #[pyfn(m, "iglob")]
    pub fn iglob(py: Python, pathname: String) -> PyResult<PyObject> {
        glob_fn(py, pathname)
    }

    #[pyfn(m, "fnmatch")]
    pub fn fnmatch(py: Python) -> PyResult<PyObject> { Ok(py.None()) }
    #[pyfn(m, "glob1")]
    pub fn glob1(py: Python) -> PyResult<PyObject> { Ok(py.None()) }

    #[pyfn(m, "glob")]
    pub fn glob_fn(py: Python, pathname: String) -> PyResult<PyObject> {
        let result: Vec<_> = glob(pathname.as_str())
            .unwrap()
            .filter_map(Result::ok)
            .map(|p| p.to_str().unwrap().to_string())
            .collect();
        Ok(result.to_object(py))
    }

    #[pyfn(m, "has_magic")]
    pub fn has_magic(py: Python) -> PyResult<PyObject> { Ok(py.None()) }

    Ok(())
}
