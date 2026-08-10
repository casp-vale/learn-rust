#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Updated variant to hold UsState data
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // Coin::Penny => 1,
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Coin::Quarter => 25,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }

}

fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let alabama_quarter = Coin::Quarter(UsState::Alabama);
    let alaska_quarter = Coin::Quarter(UsState::Alaska);

    println!("Penny value: {} cent\n", value_in_cents(penny));
    println!("Nickel value: {} cents\n", value_in_cents(nickel));
    println!("Dime value: {} cents\n", value_in_cents(dime));
    
    // Testing pattern matching with bound state values
    println!("Quarter value: {} cents\n", value_in_cents(alabama_quarter));
    println!("Quarter value: {} cents\n", value_in_cents(alaska_quarter));
}
