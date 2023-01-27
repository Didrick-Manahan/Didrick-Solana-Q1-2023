use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect(); //used to get any args that are passed in when we run "cargo run"
    let command = args[1].clone();
    let name = "Zod";
    let status = "69%";

    //println!("Args: {:?}", args);
    // first element args[0] in this "args" vector is the target of the executable ("target/debug/rust_sandbox")

    // println!("Command: {}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not a valid command! Valid commands: hello | status");
    }
}
