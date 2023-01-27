// Primitive str = immutable fixed-length string somewhere in memory
// String = growable, heap-allocated data structure - use when need modify or own string data

pub fn run() {
    let mut hello = String::from("Hello");
    //doesnt work if string literal

    // get length
    println!("Length: {}", hello.len());

    // push char
    hello.push('Z');

    // push string
    hello.push_str("od!");

    // capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // check if empty
    println!("Is Empty: {}", hello.is_empty());

    // contains
    println!("Contains 'World' {}", hello.contains("World"));

    // replace
    println!("Replace: {}", hello.replace("Zod", "There"));

    // loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('w');
    s.push('b');
    s.push('a');

    // assertion testing
    // only shows an error if it fails, shows nothing if it passes
    assert_eq!(3, s.len());
    assert_eq!(10, s.capacity());
    println!("{}", s);
}
