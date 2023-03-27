/// Regular Struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/// Tuple Struct
struct Color(i32, i32, i32);

/// Unit like struct
struct AlwaysEqual;

#[allow(unused_variables)]
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anothermail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        // active: user1.active,
        // username: user1.username,
        // sign_in_count: user1.sign_in_count,
        ..user1
    };

    let black = Color(0, 0, 0);
    
    let subject = AlwaysEqual;

    // println!("{}", user2.username)

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
