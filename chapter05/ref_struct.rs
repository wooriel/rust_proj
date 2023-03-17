struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    // erroneous
    let user1 = User {
        active: true,
        username: "someusername",
        email: "someone@zmail.com",
        sign_in_count: 1,
    };
}