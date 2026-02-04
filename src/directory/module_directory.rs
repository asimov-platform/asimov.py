// This is free and unencumbered software released into the public domain.

use asimov_directory::{ModuleDirectory as _, fs::ModuleDirectory as FsModuleDirectory};
use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyString};

/// A module directory stored on a file system (e.g., `$HOME/.asimov/modules/`).
#[pyclass(frozen, module = "asimov", str = "{0}")]
pub struct ModuleDirectory(FsModuleDirectory);

#[pymethods]
impl ModuleDirectory {
    /// Opens the default module directory in the user's home directory.
    ///
    /// On Unix platforms, including macOS and Linux, this is `$HOME/.asimov/modules/`.
    #[staticmethod]
    pub fn home() -> PyResult<Self> {
        let Ok(result) = FsModuleDirectory::home() else {
            Err(PyErr::new::<PyRuntimeError, _>(
                "Failed to open module directory", // TODO
            ))?
        };
        Ok(Self(result))
    }

    #[new]
    pub fn __new__(path: &str) -> PyResult<Self> {
        let Ok(result) = FsModuleDirectory::open(path) else {
            Err(PyErr::new::<PyRuntimeError, _>(
                "Failed to open module directory", // TODO
            ))?
        };
        Ok(Self(result))
    }

    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let class_name: Bound<'_, PyString> = slf.get_type().qualname()?;
        Ok(format!("{}({:?})", class_name, slf.borrow().0.as_str()))
    }

    /// Checks if a module is installed.
    pub fn is_installed(&self, module_name: String) -> bool {
        self.0.is_installed(module_name)
    }

    /// Checks if a module is installed and enabled.
    pub fn is_enabled(&self, module_name: String) -> bool {
        self.0.is_enabled(module_name)
    }
}
