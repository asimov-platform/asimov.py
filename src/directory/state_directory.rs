// This is free and unencumbered software released into the public domain.

use super::{ConfigDirectory, ModuleDirectory, ProgramDirectory};
use asimov_directory::{StateDirectory as _, fs::StateDirectory as FsStateDirectory};
use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyString};

/// A state directory stored on a file system (e.g., `$HOME/.asimov/`).
#[pyclass(frozen, module = "asimov", str = "{0}")]
pub struct StateDirectory(pub(crate) FsStateDirectory);

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

    /// Opens the configuration directory under this state directory.
    pub fn configs(&self) -> PyResult<ConfigDirectory> {
        let Ok(result) = self.0.configs() else {
            Err(PyErr::new::<PyRuntimeError, _>(
                "Failed to open configuration directory", // TODO
            ))?
        };
        Ok(ConfigDirectory(result))
    }

    /// Opens the module directory under this state directory.
    pub fn modules(&self) -> PyResult<ModuleDirectory> {
        let Ok(result) = self.0.modules() else {
            Err(PyErr::new::<PyRuntimeError, _>(
                "Failed to open module directory", // TODO
            ))?
        };
        Ok(ModuleDirectory(result))
    }

    /// Opens the program directory under this state directory.
    pub fn programs(&self) -> PyResult<ProgramDirectory> {
        let Ok(result) = self.0.programs() else {
            Err(PyErr::new::<PyRuntimeError, _>(
                "Failed to open program directory", // TODO
            ))?
        };
        Ok(ProgramDirectory(result))
    }
}
