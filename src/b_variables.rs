// Variables hold primitive data or reference to data
// Variables ate immutable by default
// Rust is a block-scoped language

pub fn run() {
    println!("**************b_variables.rs***************");

    let name = "Adil";
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);

    let name = 1982; // Shadowing
    age = 38;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 1;
    println!("ID: {}", ID);

    const STARTING_MISSILES: i32 = 8;
    const READY_AMOUNT: i32 = 2;
    // let mut missiles = STARTING_MISSILES;
    // let ready = READY_AMOUNT;
    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    // missiles = missiles - ready;
    missiles -= ready;
    println!("{} missiles left", missiles);

    // Assign multiple Variables
    let (my_name, my_age) = ("Adil", 37);
    println!("{} is {}", my_name, my_age);
}
