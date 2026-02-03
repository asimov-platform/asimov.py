// This is free and unencumbered software released into the public domain.

use asimov_directory::fs::ProgramDirectory as FsProgramDirectory;
use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyString};

/// A program directory stored on a file system (e.g., `$HOME/.asimov/libexec/`).
#[pyclass(frozen, module = "asimov", str = "{0}")]
pub struct ProgramDirectory(FsProgramDirectory);

#[pymethods]
impl ProgramDirectory {
    #[new]
    pub fn __new__(path: &str) -> PyResult<Self> {
        let Ok(result) = FsProgramDirectory::open(path) else {
            Err(PyErr::new::<PyRuntimeError, _>(
                "Failed to open program directory", // TODO
            ))?
        };
        Ok(Self(result))
    }

    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let class_name: Bound<'_, PyString> = slf.get_type().qualname()?;
        Ok(format!("{}({:?})", class_name, slf.borrow().0.as_str()))
    }
}
