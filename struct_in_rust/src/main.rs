// user struct

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    // new
    fn new(username: String, email: String, count: u64, status: bool) -> User {
        User {
            username,
            email,
            sign_in_count: count,
            active: status,
        }
    }

    // user details
    fn user_details(&self) {
        println!("User email: {}", self.email);
        println!("User username: {}", self.username);
        println!("User active: {}", self.active);
        println!("User sign_in_count: {}", self.sign_in_count);
    }

    // show details using format
    fn show_details(&self) -> String {
        format!(
            "User email: {}\nUser username: {}\nUser active: {}\nUser sign_in_count: {}",
            self.email, self.username, self.active, self.sign_in_count
        )
    }
}

fn main() {
    let user1 = User {
        email: String::from("Mike"),
        username: String::from("mikee"),
        active: true,
        sign_in_count: 1,
    };

    // access struct fields
    println!("User1 email: {}", user1.email);
    println!("User1 username: {}", user1.username);
    println!("User1 active: {}", user1.active);
    println!("User1 sign_in_count: {}", user1.sign_in_count);

    // create user using new
    let user2 = User::new(String::from("John"), String::from("jhon"), 2, true);

    // access struct fields using method
    user2.user_details();

    // access struct fields using method witg format
    println!("{}", user2.show_details());
}
