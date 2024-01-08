use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("non-existent_file.txt");
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                println!("{}", line.unwrap());
            }
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                // We can also use println! here
                panic!("File not found: {}", error)
            }
            _ => {
                // We can also use println! here
                panic!("Error opening file: {}", error)
            }
        },
    };
}
