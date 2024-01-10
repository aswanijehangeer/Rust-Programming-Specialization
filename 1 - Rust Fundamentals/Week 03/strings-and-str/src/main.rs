fn print_str(s: &str) {
    let new_string = format!("{} some other stuff.", s);
    println!("{}", new_string);
}

fn print_string(s: String) {
    println!("{}", s);
}

fn main() {
    let s = "Hello, world!";
    print_str(s);

    // String is growable and mutable whereas str is not.
    // String is owned by the code that creates it.
    let salutation = String::from("hello");
    print_string(salutation);
}
