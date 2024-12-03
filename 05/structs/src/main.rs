struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("User1"),
        email: String::from("user@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("another@example.com");
    println!("{}", user1.email);

    let jo = build_user(String::from("jo@example.com"), String::from("jo1"));
    println!("{}", jo.email);

    let jo2 = User {
        email: String::from("superjo@example.com"),
        ..jo
    };
    println!("{}", jo2.username);
    //println!("{}", jo.username); // This won't compile because of borrowing from jo2
}


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}