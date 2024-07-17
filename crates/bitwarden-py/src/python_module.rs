use pyo3::prelude::*;

use crate::client::BitwardenClient;

#[pymodule]
fn bitwarden_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<BitwardenClient>()?;
    Ok(())
}
