use ndarray as nd;


fn main() {
    // create array
    let mut array1: nd::Array1<f64> = nd::Array1::range(0., 10., 1.);
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

    // multi-d arrays
    let mut array2: nd::Array2<f64> = nd::Array2::random();
}
