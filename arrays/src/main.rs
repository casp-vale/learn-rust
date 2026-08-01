fn main() {

    // Compound Types
    // Arrays - multiple values with same types
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let b: [i32; 5] = [1, 2, 3, 4, 5];

    // can be accessed using index numbers of array
    println!("second element of array a: {}", a[1]);

    println!("sixth month in the array: {}", months[5]);

    println!("last element in array b of size 5: {}", b[4]);
}
