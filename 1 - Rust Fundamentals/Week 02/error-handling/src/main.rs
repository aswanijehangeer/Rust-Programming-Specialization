use std::fs::File;
use std::io::{BufRead, BufReader};

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

            std::io::ErrorKind::PermissionDenied => {
                // We can also use println! here
                panic!("Permission denied: {}", error)
            }
            _ => {
                // We can also use println! here
                panic!("Error opening file: {}", error)
            }
        },
    };
}

// use std::io::Write;

// fn write_to_file(file_path: &str, content: &str) {
//     let mut file = match File::create(file_path) {
//         Ok(file) => file,
//         Err(error) => {
//             panic!("Error creating file: {}", error)
//         }
//     };
//     match file.write_all(content.as_bytes()) {
//         Ok(_) => println!("Successfully wrote to file."),
//         Err(error) => {
//             panic!("Error writing to file: {}", error)
//         }
//     }
// }
