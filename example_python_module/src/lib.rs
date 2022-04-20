use pyo3::prelude::{pymodule, PyModule, PyResult, Python, pyfunction, wrap_pyfunction};
use numpy::{IntoPyArray, PyArray1};
use ndarray as nd;


fn _incr_array(mut array: nd::ArrayViewMut1<f64>, scalar: f64) {
    for i in 0..array.len() {
        array[i] += scalar;
    }
}

fn _mult_arrays(array1: nd::ArrayView1<f64>, array2: nd::ArrayView1<f64>) -> nd::Array1<f64> {
    &array1 * &array2
}

#[pyfunction]
fn test<'py> (py: Python<'py>, a: f64, array: &'py PyArray1<f64>) -> &'py PyArray1<f64> {
    let output: nd::Array1<f64> = nd::Array1::zeros(10);
    output.into_pyarray(py)
}

#[pymodule]
fn rust_python(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(test))?;

    #[pyfn(m)]
    #[pyo3(name = "incr_array")]
    fn incr_array<'py>(
        _py: Python<'py>,
        array: &'py PyArray1<f64>,
        scalar: f64,
    ) {
        // flipping vv_corr once and compution convolution as dot prod is faster
        let array = unsafe { array.as_array_mut() };
        _incr_array(array, scalar);
    }

    #[pyfn(m)]
    #[pyo3(name = "incr_array")]
    fn mult_arrays<'py>(
        py: Python<'py>,
        array1: &'py PyArray1<f64>,
        array2: &'py PyArray1<f64>,
    ) -> &'py PyArray1<f64> {
        // flipping vv_corr once and compution convolution as dot prod is faster
        let array1 = unsafe{ array1.as_array() };
        let array2 = unsafe{ array2.as_array() };
        _mult_arrays(array1, array2).into_pyarray(py)
    }

    Ok(())
}
