fn main() {

    // mutable and immutable
    // let mut x = 5;
    // println!("The value of x is: {x}");

    // x = 9;
    // println!("The value of x is: {x}");

    // Shadowing a variable
    let x = 5;
    let x = x + 5;

    {
        let x = x * 10;
        println!("The value of x in inner scope: {x}");
    }

    println!("The value of x is: {x}");
}
