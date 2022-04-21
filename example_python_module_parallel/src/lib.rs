use pyo3::prelude::*;
use numpy::IntoPyArray;
use numpy as np;
use ndarray as nd;
use rayon::prelude::*;


// iterate over array in parallel, compute ncorr elements of the
// correlation directly, and return the result
#[pyfunction]
fn correlation <'py> (
    py: Python<'py>,
    array1: &'py np::PyArray1<f64>,
    array2: &'py np::PyArray1<f64>,
    ncorr: usize,
) -> &'py np::PyArray1<f64> {
    // Cast python type into rust type
    let array1 = unsafe{ array1.as_array() };
    let array2 = unsafe{ array2.as_array() };
    let mut result: nd::Array1<f64> = nd::Array1::zeros(ncorr);
    // get data of result array
    let slice_result = result.as_slice_mut().unwrap();
    // Iterate over output array in parallel
    // We split result_slice into non-verlapping slices to assure every
    // and pass them to threads, to assure every piece of data is only
    // mutated by one source
    slice_result.par_iter_mut().enumerate().for_each(|(tcorr, x)| {
        // compute the correlation value for one tcorr
        let mut corr = 0.0;
        for j in 0..(array1.len() - tcorr) {
            corr += array1[j] * array2[j + tcorr];
        }
        // write the result into the output array
        *x = corr;
    });
    result.into_pyarray(py)
}

// Construct the python module, which we will be able to import
// Modle needs to return an empty PyResult
#[pymodule]
fn rust_python_parallel(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(correlation))?;
    Ok(())
}
