fn main() {
    // let maybe_number = Some(None);
    let maybe_number = Some(43);
    if let Some(number) = maybe_number {
        println!("The number is {}", number);
    } else {
        println!("There is no number.");
    }
}
