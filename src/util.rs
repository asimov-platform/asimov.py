// This is free and unencumbered software released into the public domain.

use pyo3::{PyResult, Python};
use std::{sync::OnceLock, time::Duration};
use tokio::runtime::Runtime;

pub fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    static RUNTIME_PID: OnceLock<u32> = OnceLock::new();
    let current_pid = std::process::id();
    let runtime_pid = *RUNTIME_PID.get_or_init(|| current_pid);
    if current_pid != runtime_pid {
        panic!("Forked process detected, bailing out."); // TODO
    }
    RUNTIME.get_or_init(|| Runtime::new().expect("should create the Tokio runtime"))
}

pub fn wait_for_future<F>(py: Python, fut: F) -> PyResult<F::Output>
where
    F: Future + Send,
    F::Output: Send,
{
    let runtime: &Runtime = &runtime();
    const INTERVAL_CHECK_SIGNALS: Duration = Duration::from_millis(500);

    py.run(cr"pass", None, None)?;
    py.check_signals()?;

    py.detach(|| {
        runtime.block_on(async {
            tokio::pin!(fut);
            loop {
                tokio::select! {
                    res = &mut fut => break Ok(res),
                    _ = tokio::time::sleep(INTERVAL_CHECK_SIGNALS) => {
                        Python::attach(|py| {
                                py.run(cr"pass", None, None)?;
                                py.check_signals()
                        })?;
                    }
                }
            }
        })
    })
}
