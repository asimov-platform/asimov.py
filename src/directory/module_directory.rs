// This is free and unencumbered software released into the public domain.

use super::ModuleNameIterator;
use crate::util::wait_for_future;
use asimov_directory::{ModuleDirectory as _, fs::ModuleDirectory as FsModuleDirectory};
use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyString};
use std::sync::Arc;

/// A module directory stored on a file system (e.g., `$HOME/.asimov/modules/`).
#[pyclass(frozen, module = "asimov", str = "{0}")]
pub struct ModuleDirectory(pub(crate) Arc<FsModuleDirectory>);

impl From<FsModuleDirectory> for ModuleDirectory {
    fn from(input: FsModuleDirectory) -> Self {
        Self(Arc::new(input))
    }
}

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
        Ok(Self::from(result))
    }

    #[new]
    pub fn __new__(path: &str) -> PyResult<Self> {
        let Ok(result) = FsModuleDirectory::open(path) else {
            Err(PyErr::new::<PyRuntimeError, _>(
                "Failed to open module directory", // TODO
            ))?
        };
        Ok(Self::from(result))
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

    /// Returns an iterator over the installed modules.
    fn __iter__(&self, py: Python) -> PyResult<ModuleNameIterator> {
        self.__aiter__(py)
    }

    /// Returns an iterator over the installed modules.
    fn __aiter__(&self, py: Python) -> PyResult<ModuleNameIterator> {
        let inner = Arc::clone(&self.0);
        wait_for_future(py, async move {
            let it = inner.iter_installed().await.unwrap();
            ModuleNameIterator::from(it)
        })
    }
}
