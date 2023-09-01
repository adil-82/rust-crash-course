// Tuples group together values of different types
// Max 12 element

pub fn run() {
    println!("**************e_tuples.rs***************");

    let someone = ("Adil", "Casa", 37);
    println!("{} is from {} and is {}", someone.0, someone.1, someone.2);

    let person: (&str, &str, i8) = ("Adil", "Casa", 37);
    println!("{} is from {} and is {}", person.0, person.1, person.2);

    // destructuring:
    let (_name, _state, _age) = someone;
    println!("{} is from {} and is {}", _name, _state, _age);

    // dot-notation:
    let name = person.0;
    let state = person.1;
    let age = person.2;
    println!("{} is from {} and is {}", name, state, age);

}
