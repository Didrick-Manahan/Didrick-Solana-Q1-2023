// arrays - fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    //re-assign value (cannot add on! only reassign)
    numbers[2] = 20;

    //use debug notation
    println!("{:?}", numbers);

    //get single val
    println!("Single Value: {}", numbers[0]);

    //get array length
    println!("Array Length: {}", numbers.len());

    //arrays are stack allocated (reference to array )
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers[2..];
    println!("Slice: {:?}", slice);
}
