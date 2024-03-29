/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays(fixed length)
Vectors<dyamic>
*/

pub fn run() {
    //Default is i32
    let x = 1;

    //Default is f64
    let y = 2.5;

    //Add explicit type
    let z: i64 = 3333333333333;

    //Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean
    let is_active: bool = true;
    //let is_active = false;

    //Get boolean from expression
    let is_greater: bool = 1000.0 < 5.00;
    let a1 = 'Z';
    let face = '\u{1F703}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
