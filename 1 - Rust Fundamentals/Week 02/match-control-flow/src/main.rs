use std::io;

fn main() {
    // let name = "Jehangeer";

    // using match expression to pattern match against variable 'name'
    // match name {
    //     "Good Bye" => println!("Sorry to see you go."),
    //     "Jehangeer" => println!("Hi Jehangeer, Nice to meet you."),
    //     _ => println!("I can't find a greeting, good bye."),
    // }

    println!("Please enter a greeting:");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    // using match expression to pattern match against variable 'name'
    match name.trim() {
        "Good Bye" => println!("Sorry to see you go."),
        "Hello" => println!("Hi, Nice to meet you."),
        _ => println!("I can't find a greeting, good bye."),
    }
}
