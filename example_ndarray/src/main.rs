use ndarray as nd;
use ndarray_rand::rand_distr::StandardNormal;
use ndarray_rand::RandomExt;


fn main() {
    // create array
    let array1: nd::Array1<f64> = nd::Array1::range(0., 10., 1.);
    println!("Sum array1: {:?}", array1.sum());

    // add 1. to all elements
    let array1_incr = &array1 + 1.;
    println!("Sum array1: {:?}", array1_incr.sum());


    // add range(10) / 2. to the array
    let update: nd::Array1<f64> = nd::Array1::range(0., 5., 0.5);
    let new_array = &array1 + &update;
    let array1 = array1 + &update;
    println!("Sum array1: {:?}", new_array.sum());
    println!("Sum array1: {:?}", array1.sum());

    // add 2. to all elements inplace
    let array1_incr = array1 + 1.;
    println!("Sum array1: {:?}", array1_incr.sum());

    // multi-d arrays
    let mut array2: nd::Array2<f64> = nd::Array2::random((5, 100), StandardNormal);
    // get single value by passing array [row, column] to indexing operator array[...]
    println!("array2[0, 0]: {:?}", array2[(0, 0)]);
    // get single row, column by convenience funciton
    println!("\n\narray2.row(0): {:?} \n\n", array2.row(0));
    // As always, one can also get mutable equivalent
    array2.column_mut(1).assign(&nd::Array1::zeros((5,)));
    println!("array2.column(1): {:?}\n\n", array2.column(1));
    // general slicing, get the rows except the last two and the colums starting at 90
    let slice1: nd::ArrayView2<f64> = array2.slice(nd::s![..-2, 90..]);
    println!("slice1: {:?}", slice1.shape());
    // general slicing, get every row and every 21th element of the rows
    let slice2: nd::ArrayView2<f64> = array2.slice(nd::s![.., ..;21]);
    println!("slice2: {:?}", slice2.shape());
}
