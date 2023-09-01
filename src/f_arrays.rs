// Array - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
    println!("**************e_tuples.rs***************");

    // let mut numbers: [i32; 4 ] = [1, 2, 3, 4];
    // let mut numbers: [i8; 4 ] = [1, 2, 3, 4];
    let mut numbers = [1, 2, 3, 4];

    // Re-assign value
    println!("numbers: {:?}", numbers);
    numbers[2] = 20;
    println!("numbers: {:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    // let slice: &[i32] = &numbers[1..3];
    let slice = &numbers[1..3];
    println!("Slice [1..3]: {:?}", slice);

    let byte = [1; 8];
    println!("byte: {:?}", byte);
}
