// This is free and unencumbered software released into the public domain.

use crate::util::wait_for_future;
use asimov_directory::{ModuleNameIterator as _, fs::ModuleNameIterator as FsModuleNameIterator};
use pyo3::{
    exceptions::{PyStopAsyncIteration, PyStopIteration},
    prelude::*,
};
use std::sync::Arc;
use tokio::sync::Mutex;

/// An iterator over module names in the module directory.
#[pyclass(frozen, module = "asimov", str = "ModuleNameIterator")]
pub struct ModuleNameIterator(pub(crate) Arc<Mutex<FsModuleNameIterator>>);

impl From<FsModuleNameIterator> for ModuleNameIterator {
    fn from(input: FsModuleNameIterator) -> Self {
        Self(Arc::new(Mutex::new(input)))
    }
}

#[pymethods]
impl ModuleNameIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self, py: Python) -> PyResult<String> {
        let inner = Arc::clone(&self.0);
        let next = async move {
            let mut lock = inner.lock().await;
            let item = lock.next().await;
            drop(lock);
            match item {
                Some(name) => Ok(name),
                None => Err(PyStopIteration::new_err("")),
            }
        };
        wait_for_future(py, next).flatten()
    }

    fn __anext__<'py>(&'py self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.0);
        let anext = async move {
            let mut lock = inner.lock().await;
            let item = lock.next().await;
            drop(lock);
            match item {
                Some(name) => Ok(name),
                None => Err(PyStopAsyncIteration::new_err("")),
            }
        };
        pyo3_async_runtimes::tokio::future_into_py(py, anext)
    }
}
