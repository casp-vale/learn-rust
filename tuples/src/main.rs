fn main() {

    // Compound types
    // Tuple - grouping multiple values with multiple types
    // a tuple x, can also be accessed using indices like x.0, x.1, x.2
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("Value of x: {x}");
    println!("Value of y: {y}");
    println!("Value of z: {z}");
}
