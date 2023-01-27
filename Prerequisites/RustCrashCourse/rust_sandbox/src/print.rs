pub fn run() {
    //print to console
    println!("Hello from the print.rs file");

    //Basic Formatting
    println!("{} was born in {}", "Zod", "Canada");

    //Positional Arguements
    println!(
        "{0} is from {1} and {0} loves {2}!",
        "Zod", "Canada", "Rust"
    );

    //Named Arguements
    println!(
        "{name} likes to play {activity}",
        name = "Zod",
        activity = "Tennis"
    );

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
