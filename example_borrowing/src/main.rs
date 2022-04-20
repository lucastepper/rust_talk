#![allow(unused)]

fn get_sum(data: &Vec<f64>) -> f64 {
    let mut sum = 0.;
    for x in data {
        sum += *x;
    }
    return sum
}

fn get_sum_take_own(data: Vec<f64>) -> f64 {
    let mut sum = 0.;
    for x in data.iter() {
        sum += *x;
    }
    return sum
}

fn main() {
    let data = vec![0., 1., 2., 4.];
    let sum1 = get_sum(&data);
    let sum2 = get_sum_take_own(data);
    println!("The sum is {}", sum1);
    println!("The sum is still {}", sum2);

    // Control where data is stored
    // stored in programm, static size, faster to work with
    let data = [0., 1., 2., 4.];
    // dynamic sized, region for additional data "Heap"
    let data = vec![0., 1., 2., 4.];
    // Same for strings, static strings
    let string_slice = "Hello, World!";
    // dynamic strings
    let string = String::from("Hello, World!");
}
