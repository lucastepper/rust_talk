// ##### Some Numeric Types #####
let types_num: [u8, u64, i8, i64, f32, f64, usize];
// ##### Some Composite Types #####
let types_comp: [Array, Tuple, Vec, String, &str, HashMap];
// ##### Define var with type #####
let var_num: u8 = 0;
// ##### Define var with type #####
let vec: Vec<f32> = vec![1.0, 2.0, 3.0];
// ##### Type inference possible -> default: f64 #####
let vec = vec![1.0, 2.0, 3.0];

// ##### Function definition #####
fn add(a: i32, b: i32) -> i32 {
    a + b
    // same as return a + b
}
