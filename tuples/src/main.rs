fn main() {

    // Compound types
    // Tuple - grouping multiple values with multiple types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("Value of x: {x}");
    println!("Value of y: {y}");
    println!("Value of z: {z}");
}
