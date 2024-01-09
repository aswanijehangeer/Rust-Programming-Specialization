#[derive(Debug)]

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    email: String,
    phone_number: String,
}

fn main() {
    println!(
        "{:?}",
        Person {
            first_name: "Jehangeer".to_string(),
            last_name: "Aswani".to_string(),
            age: 23,
            email: "aswanijehangeer@gmail.com".to_string(),
            phone_number: "+923353711462".to_string(),
        }
    );
}
