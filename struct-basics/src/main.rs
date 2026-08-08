 struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

fn main() {
    
    let user1 = User {
        active: false,
        username: String::from("user1-name"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };

    println!("User1 Details: ");
    println!("active: {}", user1.active);
    println!("username: {}", user1.username);
    println!("email: {}", user1.email);
    println!("Sign in count: {}", user1.sign_in_count);

    let user2 = build_user("user2@example.com".to_string(), "user2-name".to_string());
    println!(" ");
    println!("User2 Details: ");
    println!("active: {}", user2.active);
    println!("username: {}", user2.username);
    println!("email: {}", user2.email);
    println!("Sign in count: {}", user2.sign_in_count);

    let user3 = User {
        username: String::from("user3-name"),
        email: String::from("user3@example.com"),
        ..user1
    };
    println!(" ");
    println!("User3 Details: ");
    println!("active: {}", user3.active);
    println!("username: {}", user3.username);
    println!("email: {}", user3.email);
    println!("Sign in count: {}", user3.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // same as username: username,
        email, //same as email: email,
        sign_in_count: 1,
    }
}