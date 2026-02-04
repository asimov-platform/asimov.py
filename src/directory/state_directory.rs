// This is free and unencumbered software released into the public domain.

use asimov_directory::{StateDirectory as _, fs::StateDirectory as FsStateDirectory};
use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyString};

/// A state directory stored on a file system (e.g., `$HOME/.asimov/`).
#[pyclass(frozen, module = "asimov", str = "{0}")]
pub struct StateDirectory(FsStateDirectory);

#[pymethods]
impl StateDirectory {
    /// Opens the default state directory in the user's home directory.
    ///
    /// On Unix platforms, including macOS and Linux, this is `$HOME/.asimov/`.
    #[staticmethod]
    pub fn home() -> PyResult<Self> {
        let Ok(result) = FsStateDirectory::home() else {
            Err(PyErr::new::<PyRuntimeError, _>(
                "Failed to open state directory", // TODO
            ))?
        };
        Ok(Self(result))
    }

    #[new]
    pub fn __new__(path: &str) -> PyResult<Self> {
        let Ok(result) = FsStateDirectory::open(path) else {
            Err(PyErr::new::<PyRuntimeError, _>(
                "Failed to open state directory", // TODO
            ))?
        };
        Ok(Self(result))
    }

    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let class_name: Bound<'_, PyString> = slf.get_type().qualname()?;
        Ok(format!("{}({:?})", class_name, slf.borrow().0.as_str()))
    }

    /// Checks if any configurations are available.
    pub fn has_configs(&self) -> bool {
        self.0.has_configs()
    }

    /// Checks if any modules are installed.
    pub fn has_modules(&self) -> bool {
        self.0.has_modules()
    }

    /// Checks if any programs are installed.
    pub fn has_programs(&self) -> bool {
        self.0.has_programs()
    }
}
