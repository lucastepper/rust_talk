use pyo3::prelude::*;
use numpy::IntoPyArray;
use numpy as np;
use ndarray as nd;


fn _incr_array(mut array: nd::ArrayViewMut1<f64>, scalar: f64) {
    for i in 0..array.len() {
        array[i] += scalar;
    }
}

// muatate a numpy array inplace
// 'py is the lifetime of the python interpreter
// we have to explicitly state that array lives as long as the python
// interpreter lives, and that the function is generic over that lifetime
#[pyfunction]
fn increment_array <'py> (array: &'py np::PyArray1<f64>, scalar: f64) {
    // cast python type into rust type
    let mut array = unsafe{ array.as_array_mut() };
    // Do mutation
    for i in 0..array.len() {
        array[i] += scalar;
    }
}

// Computation based on numpy arrays, returning a new array
// For this, we take the python interpreter as a parameter
// which lets us create a new numpy array as output in the memory
// space of the python interpreter
#[pyfunction]
fn mult_arrays <'py> (
    py: Python<'py>,
    array1: &'py np::PyArray1<f64>,
    array2: &'py np::PyArray1<f64>,
) -> &'py np::PyArray1<f64>{
    // Cast python type into rust type
    let array1 = unsafe{ array1.as_array() };
    let array2 = unsafe{ array2.as_array() };
    // Mutlpy the two arrays element-wise. Since we do not take onwership
    // of array1 or array2, we have to do a copy of the data here,
    // so output is in a new mermory location.
    let output = &array1 * &array2;
    // cast rust type into python type
    output.into_pyarray(py)
}

// Construct the python module, which we will be able to import
// Modle needs to return an empty PyResult
#[pymodule]
fn rust_python(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(increment_array))?;
    m.add_wrapped(wrap_pyfunction!(mult_arrays))?;
    Ok(())
}
