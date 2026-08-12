// // The crate root file defines the top-level modules.
// // Module 1: Front of House (Handles customer interaction)
// pub mod front_of_house {
//     // Nested module: Hosting
//     pub mod hosting {
//         pub fn add_to_waitlist() {
//             println!("Customer added to waitlist!");
//         }

//         fn seat_at_table() {
//             println!("Seated at table.");
//         }
//     }

//     // Nested module: Serving
//     mod serving {
//         fn take_order() {}
//         fn serve_order() {}
//         fn take_payment() {}
//     }
// }

// // Module 2: Back of House (Handles kitchen and inventory)
// pub mod back_of_house {
//     // A public struct with mixed field visibility
//     pub struct Breakfast {
//         pub toast: String,      // Public field (customer chooses toast type)
//         seasonal_fruit: String, // Private field (chef chooses available fruit)
//     }

//     impl Breakfast {
//         // Associated function (constructor) required because `seasonal_fruit` is private
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }

//     // A public Enum: Making an enum public automatically makes ALL its variants public
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }

//     fn fix_incorrect_order() {
//         cook_order();
//         // `super` refers to the parent scope (crate root in this case)
//         super::deliver_order();
//     }

//     fn cook_order() {}
// }

// // Function at the crate root level
// fn deliver_order() {
//     println!("Delivering order to table...");
// }

// // Public interface function demonstrating paths
// pub fn eat_at_restaurant() {
//     // Absolute Path (starts from crate root)
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative Path (starts from current module scope)
//     front_of_house::hosting::add_to_waitlist();

//     // Order a breakfast in the summer with Rye toast
//     let mut meal = back_of_house::Breakfast::summer("Rye");

//     // Change the public `toast` field
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);

//     // Order appetizers (Enum variants are public by default)
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }



// Declare that the 'front_of_house' module exists in another file/folder
pub mod front_of_house;

pub mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// Bring `hosting` into local scope so users can call `hosting::add_to_waitlist()` directly
use front_of_house::hosting;

// Re-exporting (`pub use`): External code calling this library can now call
// `restaurant::hosting::add_to_waitlist()` directly!
pub use front_of_house::hosting as PublicHosting;

pub fn eat_at_restaurant() {
    // Thanks to the `use` statement above, we don't need full paths here:
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
}