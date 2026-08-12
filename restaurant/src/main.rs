// Import our library crate ('restaurant' is the package name in Cargo.toml)
use restaurant::eat_at_restaurant;
use restaurant::PublicHosting; // Using our re-exported `pub use` path!

fn main() {
    println!("=== Welcome to the Restaurant ===");

    // Call library functions
    eat_at_restaurant();

    // Use the re-exported module path directly
    PublicHosting::add_to_waitlist();
}