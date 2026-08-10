#[derive(Debug)]
enum IpAddr {
    V4(String), // Variant holding IPv4 address string (field 0)
    V6(String), // Variant holding IPv6 address string (field 1)
}

fn main() {
    // Construct both V4 and V6 variants
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // Print using pattern matching with `match`
    println!("--- Printing using match ---");
    print_ip_address(&home);
    print_ip_address(&loopback);

    // Print using pattern matching with `if let`
    println!("\n--- Printing using if let ---");
    if let IpAddr::V4(address) = &home {
        println!("IPv4 Address: {address}");
    }

    if let IpAddr::V6(address) = &loopback {
        println!("IPv6 Address: {address}");
    }
}

// Helper function to extract and print field 0 from any variant
fn print_ip_address(ip: &IpAddr) {
    match ip {
        IpAddr::V4(address) => println!("Found IPv4: {address}"),
        IpAddr::V6(address) => println!("Found IPv6: {address}"),
    }
}