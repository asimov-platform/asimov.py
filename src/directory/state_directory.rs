// This is free and unencumbered software released into the public domain.

use asimov_directory::fs::StateDirectory as FsStateDirectory;
use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyString};

/// A state directory.
#[pyclass(frozen, module = "asimov", str = "{0}")]
pub struct StateDirectory(FsStateDirectory);

#[pymethods]
impl StateDirectory {
    #[new]
    pub fn __new__(path: &str) -> PyResult<Self> {
        let Ok(result) = FsStateDirectory::open(path) else {
            Err(PyErr::new::<PyRuntimeError, _>(
                "Failed to open state directory", // TODO
            ))?
        };
        Ok(Self(result))
    }

    fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let class_name: Bound<'_, PyString> = slf.get_type().qualname()?;
        Ok(format!("{}({:?})", class_name, slf.borrow().0.as_str()))
    }
}
