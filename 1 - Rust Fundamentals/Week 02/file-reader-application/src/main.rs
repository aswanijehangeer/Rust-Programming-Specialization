use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Creating a path to the `hello.txt` file
    let path = Path::new("hello.txt");
    let display = path.display();

    // Opening the path in read-only mode
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Reading the file contents into a string,
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
}
