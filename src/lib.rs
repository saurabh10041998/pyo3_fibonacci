use pyo3::prelude::*;

/// calculate the nth Fibonacci number
#[pyfunction]
fn fibonacci(n: u64) -> PyResult<u64> {
    if n <= 1 {
        return Ok(n);
    }
    
    let (mut a, mut b) = (0u64, 1u64);

    for _ in 1..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    Ok(b)
}

#[pymodule]
fn pyo3_fibonacci(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_run() {
        assert!(fibonacci(0).is_ok());
        assert_eq!(fibonacci(0).unwrap(), 0);
        assert!(fibonacci(1).is_ok());
        assert_eq!(fibonacci(1).unwrap(), 0);
        assert!(fibonacci(2).is_ok());
        assert_eq!(fibonacci(2).unwrap(), 0);
    }

    #[test]
    fn test_long_run() {
        assert!(fibonacci(50).is_ok());
        assert_eq!(fibonacci(50).unwrap(), 12586269025);

    }
}