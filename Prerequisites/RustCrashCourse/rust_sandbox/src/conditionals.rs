// conditional used to check the condition of something and act on the result (if-else just like in other languages)

pub fn run() {
    let age: u8 = 22;
    let check_id: bool = true;
    let knows_person_of_age = true;

    // if/else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drank?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave fam!");
    } else {
        println!("Bartender: I'll need to see your ID please!");
    }

    // Shorthand "if"
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is Of Age: {}", is_of_age)
}
