use pyo3::prelude::*;

use crate::client::BitwardenClient;

#[pymodule]
fn bitwarden_py(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<BitwardenClient>()?;
    Ok(())
}
