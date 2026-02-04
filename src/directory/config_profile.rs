// This is free and unencumbered software released into the public domain.

use asimov_core::Named;
use asimov_directory::fs::ConfigProfile as FsConfigProfile;
use pyo3::{prelude::*, types::PyString};

/// A configuration profile stored on a file system (e.g., `$HOME/.asimov/configs/default/`).
#[pyclass(frozen, module = "asimov", str = "{0}")]
pub struct ConfigProfile(pub(crate) FsConfigProfile);

#[pymethods]
impl ConfigProfile {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let class_name: Bound<'_, PyString> = slf.get_type().qualname()?;
        Ok(format!("{}({:?})", class_name, slf.borrow().0.as_str()))
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name().into_owned()
    }
}
