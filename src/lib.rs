// This is free and unencumbered software released into the public domain.

#[pyo3::pymodule]
mod sdk {
    use pyo3::prelude::*;

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add("__version__", env!("CARGO_PKG_VERSION"))?;
        m.add("__version_tuple__", (26, 0, 0, Some("dev1")))?; // TODO
        Ok(())
    }
}
