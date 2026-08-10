#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation block for Rectangle 
// Everything within this impl block will be associated with the Rectangle type 
impl Rectangle {

    // using &self instead of rectangle: &Rectangle.
    // the &self is actually short for self: &Self.
    // within an impl block, the type Self is an alias for the type that the impl block is for.
    // & in front of the self shorthand indicates that this method borrows the Self instance.
    // methods can take ownership of self, borrow self immutably, as we’ve done here, 
    // or borrow self mutably, just as they can any other parameter.
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}