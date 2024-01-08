// fn split_string(s: String, delimiter: char, field: usize) -> String {
//     let parts: Vec<&str> = s.split(delimiter).collect();
//     let result = parts.get(field);
// This will not compile
// result.to_string();
//     result.expect("oops! something went wrong").to_string()
// }

// function using argument
fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

use std::io;

fn main() {
    //     let chunk = split_string("hello,world".to_string(), ',', 1);
    //     println!("Split string: {}", chunk);

    //     let numbers = [1, 2, 3, 4, 5];
    //     let result = sum(&numbers);
    //     println!("The sum is: {}", result);

    let mut input = String::new();
    println!("Enter the number of elements:");

    io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();
    let mut numbers = Vec::new();

    for i in 0..n {
        let mut input = String::new();
        println!("Enter element {}: ", i + 1);
        io::stdin().read_line(&mut input).unwrap();
        let number: i32 = input.trim().parse().unwrap();
        numbers.push(number);
    }

    let result = sum(&numbers);
    println!("The sum is {}", result);
}
