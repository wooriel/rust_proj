struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("anonymous"),
        email: String::from("anonymous@zmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("another@zmail.com");

    // let user2_email = String::from("number2@zmail.com");
    // let mut user2 = build_user(user2_email, String::from("heehee"));

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@zmail.com"),
        sign_in_count: user1.sign_in_count,
    }; // user1 active, username, sign_in_count is moved into user2

    let user3 = User {
        email: String::from("third@zmail.com"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}