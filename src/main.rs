struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("nakul"),
        email: String::from("71.nakul@gmail.com"),
        sign_in_count: 1,
    };
    println!("User 1 username: {:?}", user1.username);
    println!("User 1 username: {:?}", user1.active);
    println!("User 1 username: {:?}", user1.email);
    println!("User 1 username: {:?}", user1.sign_in_count);
}

