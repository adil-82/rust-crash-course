// Scalar Data Types--
// Integers: u8, i8, u16, u32, u64, i64, u128, i128 (number of bits they take in memory)
// Floats: f32, f64
// Boolean (bool)
// Characters (char)
// Compound Data Type:
// Tuples
// Arrays

// Rust is a statically typed language, which means taht it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run() {
    println!("**************c_types.rs***************");

    // Default is "i32"
    let _x = 1;

    // Default is "f64"
    let _y = 2.5;

    // Add explicit type
    let _z: i64 = 454544545445;

    // Find max size:
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 < 5;

    let a1 = 'a';
    let face = '\u{1F600}';
    let herb = '🌿';

    println!("{:?}", (_x, _y, _z, is_active, is_greater, a1, face, herb));
}
