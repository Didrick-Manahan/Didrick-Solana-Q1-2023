//Variables hold primitive data or references to data
//Variables are immutable by default
//Rust is a block-scoped language

pub fn run() {
    let name = "Zod";
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);
    age = 38;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple vars (tuple here)
    let (my_name, my_age) = ("Brad", 37);
    print!("{} is {}", my_name, my_age);
}
