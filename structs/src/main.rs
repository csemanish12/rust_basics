fn main() {
    
    // structs
    struct Book{
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }
    
    #[derive(Debug)]
    struct User{
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // can create instance of the struct
    let mut user1 = User{
        active: true,
        username: String::from("first_user"),
        email: String::from("first_user@email.com"),
        sign_in_count: 1,
    };

    // update one field
    user1.email = String::from("first_user_updated@email.com");
    println!("User email is: {}", user1.email);

    // return struct from a function
    fn build_user(email: String, username: String) -> User {
        User{
            active: true,
            email,        // when variable name matches the key name, no need to provide key name
            username,
            sign_in_count: 1

        }
    }

    // create instance from another instance
    let user2: User = User{
        email: String::from("user2@email.com"),
        ..user1
    };
    println!("user 2: {:?}", user2);
    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let white = Color(255,255,255);

    // unit-like struct
    struct AlwaysEqual;
    let subject = AlwaysEqual;

}