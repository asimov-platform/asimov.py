// This is free and unencumbered software released into the public domain.

use asimov_directory::fs::ModuleDirectory as FsModuleDirectory;
use pyo3::prelude::*;

/// TODO
#[pyclass(frozen, module = "asimov", str = "{0}")]
pub struct ModuleNameIterator(pub(crate) FsModuleDirectory);

#[pymethods]
impl ModuleNameIterator {}
