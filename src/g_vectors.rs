// Vectors - Resizable arrays

use std::mem;

pub fn run() {
    println!("**************g_vectors.rs***************");

    // let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    let mut numbers = vec![1, 2, 3, 4];

    // Re-assign value
    numbers[2] = 20;
    println!("{:?}", numbers);

    // Add on to Vectors
    numbers.push(5);
    numbers.push(6);
    println!("{:?}", numbers);

    // Pop off last value
    numbers.pop();
    println!("{:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Get vectors length:
    println!("Vector length: {}", numbers.len());

    // Vectors are heap allocated:
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice = &numbers[1..3];
    // let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector value
    for x in numbers.iter() {
        println!("Numbers: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        // *x = *x * 2;
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}
