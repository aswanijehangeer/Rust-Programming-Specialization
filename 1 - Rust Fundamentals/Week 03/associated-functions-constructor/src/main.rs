struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main() {
    let mut new_user = User::new(
        String::from("aswanijehangeer"),
        String::from("aswanijehangeer@gmail.com"),
        String::from("https://aswanijehangeer.com"),
    );

    println!("Hello, {}", new_user.username);
    println!(
        "Account {} status is: {}",
        new_user.username, new_user.active
    );
    new_user.deactivate();
    println!(
        "Account {} status is: {}",
        new_user.username, new_user.active
    );
}

// Challenges question

// #[derive(Debug)]
// struct User {
//     username: String,
//     email: String,
//     uri: String,
//     active: bool,
// }

// impl User {
//     fn new(username: String, email: String, uri: String) -> Self {
//         Self {
//             username,
//             email,
//             uri,
//             active: true,
//         }
//     }

//     fn from_email(email: &str) -> Self {
//         let username = email.split('@').next().unwrap_or_default().to_string();
//         Self::new(username, email.to_string(), String::new())
//     }

//     fn deactivate(&mut self) {
//         self.active = false;
//     }

//     fn update_uri(&mut self, new_uri: String) {
//         self.uri = new_uri;
//     }
// }

// fn main() {
//     let mut new_user = User::new(
//         String::from("aswanijehangeer"),
//         String::from("aswanijehangeer@gmail.com"),
//         String::from("https://aswanijehangeer.com"),
//     );

//     println!("Hello, {}", new_user.username);
//     println!(
//         "Account {} status is: {}",
//         new_user.username, new_user.active
//     );

//     new_user.deactivate();
//     println!(
//         "Account {} status is: {}",
//         new_user.username, new_user.active
//     );

//     let derived_user = User::from_email("john@example.com");
//     println!("{:?}", derived_user);

//     new_user.update_uri(String::from("https://updated-uri.com"));
//     println!("Updated URI: {}", new_user.uri);
// }
