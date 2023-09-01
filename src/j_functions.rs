fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    // let sum = n1 + n2;
    // return sum
    // sum
    n1 + n2
}

pub fn run() {
    println!("**************j_functions.rs***************");

    greeting("Hello", "Adil");

    // Bind function values to variables
    let sum = add(5, 5);
    println!("Sum: {}", sum);

    // Closure:
    // let n3: i32 = 10;
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C sum: {}", add_nums(3, 3));

}
