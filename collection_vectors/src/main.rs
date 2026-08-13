fn main() {

    // creating new, empty vector to hold values of type i32
    // let mut v: Vec<i32> = Vec::new();

    // insert vlues in vector v
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // creating a new vector containing values
    // vec! macro is used 
    let v = vec![1, 2, 3, 4, 5];

    // two ways to reference a value stored in a vector
    // 1. via indexing
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // 2. via get() method
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // vactor to store a list of items of different types
    // using enums
    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];

}
