use pyo3::prelude::*;

#[pyfunction]
fn add(left: usize, right: usize) -> PyResult<usize> {
    Ok(left + right)
}

#[pymodule]
#[pyo3(name = "rust_lib")]
fn module_extension(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result.unwrap(), 4);
    }
}
