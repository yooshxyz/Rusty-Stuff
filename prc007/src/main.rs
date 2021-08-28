
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_another(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    let user2 = build_another(
        String::from("baranpasa999@gmail.com"),
        String::from("YooshXYZ")
    );

    let user3 = User {
        email: String::from("anotherexample@gmail.com"),
        username: String::from("Capo"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user4 = User {
        email: String::from("Evenmore@aol.com"),
        username: String::from("anotherExampleUsername44"),
        ..user2
    };
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
}
