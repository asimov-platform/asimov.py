// This is free and unencumbered software released into the public domain.

//#![allow(unused)]

#[pyo3::pymodule]
mod sdk {
    use pyo3::prelude::*;

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add("__version__", env!("CARGO_PKG_VERSION"))?;
        m.add("__version_tuple__", (26, 0, 0, Some("dev1")))?; // TODO: 26.0.0.dev1
        Ok(())
    }

    #[pymodule_export]
    use super::ConfigDirectory;

    #[pymodule_export]
    use super::ConfigProfile;

    #[pymodule_export]
    use super::ModuleDirectory;

    #[pymodule_export]
    use super::ModuleNameIterator;

    #[pymodule_export]
    use super::ProgramDirectory;

    #[pymodule_export]
    use super::StateDirectory;
}

mod directory;
use directory::*;

mod util;
