// functions used to store blocks of code for re-use

pub fn run() {
    greeting("Hello", "Jeff, Nate, Richard, and Chauncey");

    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure (can use outside variables which you can't do with functions, since its block-scoped!)
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3, 4));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2 //no semi-colon = expression with a return value
}
