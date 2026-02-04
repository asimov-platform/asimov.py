// This is free and unencumbered software released into the public domain.

use asimov_directory::fs::ConfigDirectory as FsConfigDirectory;
use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyString};

/// A configuration directory stored on a file system (e.g., `$HOME/.asimov/configs/`).
#[pyclass(frozen, module = "asimov", str = "{0}")]
pub struct ConfigDirectory(FsConfigDirectory);

#[pymethods]
impl ConfigDirectory {
    /// Opens the default configuration directory in the user's home directory.
    ///
    /// On Unix platforms, including macOS and Linux, this is `$HOME/.asimov/configs/`.
    #[staticmethod]
    pub fn home() -> PyResult<Self> {
        let Ok(result) = FsConfigDirectory::home() else {
            Err(PyErr::new::<PyRuntimeError, _>(
                "Failed to open configuration directory", // TODO
            ))?
        };
        Ok(Self(result))
    }

    #[new]
    pub fn __new__(path: &str) -> PyResult<Self> {
        let Ok(result) = FsConfigDirectory::open(path) else {
            Err(PyErr::new::<PyRuntimeError, _>(
                "Failed to open configuration directory", // TODO
            ))?
        };
        Ok(Self(result))
    }

    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let class_name: Bound<'_, PyString> = slf.get_type().qualname()?;
        Ok(format!("{}({:?})", class_name, slf.borrow().0.as_str()))
    }
}
