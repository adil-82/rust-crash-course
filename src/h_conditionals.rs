pub fn run() {
    println!("**************h_conditionals.rs***************");
    let age: u8 = 22;
    let chec_id: bool = true;
    let knows_person_of_age = true;

    // IF/ELSE:
    if age >= 21 && chec_id || knows_person_of_age {
        println!("What would you like to drink");
    } else if age < 21 && chec_id {
        println!("You have to leave!");
    } else {
        println!("I'll need to see your ID");
    }

    // Shorthand IF:
    // let is_of_age = if age >= 21 { true } else { false };
    let is_of_age = age >= 21 ;
    println!("Is Of Age: {}", is_of_age);
}
