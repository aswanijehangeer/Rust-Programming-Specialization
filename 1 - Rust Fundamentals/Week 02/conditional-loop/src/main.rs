// fn main() {
//     // let maybe_number = Some(None);
//     let maybe_number = Some(43);
//     if let Some(number) = maybe_number {
//         println!("The number is {}", number);
//     } else {
//         println!("There is no number.");
//     }
// }

fn main() {
    // Adding an additional level of nesting to maybe_number
    // Let maybe_number = Some(Some(42));
    let maybe_number = Some(Some(43)); // Let can change the inner value here

    if let Some(inner) = maybe_number {
        // Check the inner option
        if let Some(number) = inner {
            // Handle inner Some value
            if number == 42 {
                println!("The inner number is the Answer: {}", number);
            } else {
                println!("The inner number is {}", number);
            }
        } else {
            println!("There is no inner number.");
        }
    } else {
        println!("There is no number.");
    }
}
