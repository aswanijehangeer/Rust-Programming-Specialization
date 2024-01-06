fn main() {
    // control flow
    let proceed = true;
    if proceed {
        println!("Proceeding");
    } else {
        println!("Not proceeding");
    }

    let height = 190;
    if height > 180 {
        println!("Tall");
    } else if height > 170 {
        println!("Average");
    } else {
        println!("Short");
    }

    // challenge questions
    let is_male = true;
    let age = 25;

    if age < 18 {
        if is_male {
            println!("Young boy");
        } else {
            println!("Young girl");
        }
    } else if age >= 18 && age < 60 {
        if is_male {
            println!("Adult man");
        } else {
            println!("Adult woman");
        }
    } else {
        if is_male {
            println!("Elderly man");
        } else {
            println!("Elderly woman");
        }
    }
}
