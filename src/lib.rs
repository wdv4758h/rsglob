#![feature(proc_macro)]
#![feature(proc_macro_path_invoc)]

extern crate pyo3;
extern crate glob;


use pyo3::prelude::*;
use glob::{glob_with, MatchOptions};
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
        let options = MatchOptions {
            case_sensitive: true,
            require_literal_separator: false,
            // don't show files with '.' prefix, unless it's literally in the pattern
            require_literal_leading_dot: true,
        };
        let result: Vec<_> = glob_with(pathname.as_str(), &options)
            .unwrap()
            .filter_map(Result::ok)
            .filter_map(|p| {
                // FIXME: better solution for get rid of "./." and "./.." with ".*" pattern ?
                let s = p.to_str().unwrap();
                match s {
                    "./." | "./.." => None,
                    string @ _ => Some(string.to_string())
                }
            })
            .collect();
        Ok(result.to_object(py))
    }

    #[pyfn(m, "has_magic")]
    pub fn has_magic(py: Python) -> PyResult<PyObject> { Ok(py.None()) }

    Ok(())
}
