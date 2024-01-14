use pyo3::prelude::*;

#[pyfunction]
pub fn add(left: usize, right: usize) -> PyResult<usize> {
    Ok(left + right)
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
