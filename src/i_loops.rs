pub fn run() {
    println!("**************i_loops.rs***************");

    let mut count = 0;

    // Infinite Loop:
    loop {
        count += 1;
        println!("Count: {}", count);
        if count == 20 {
            break;
        }
    }

    // While Loop:

    let mut number = 3;
    while number != 0 {
        println!("Number {}", number);
        number -= 1;
    }

    // while count <= 100 {
    //     if count % 15 == 0 {
    //         println!("fizzbuzz");
    //     } else if count % 3 == 0 {
    //         println!("fizz");
    //     } else if count % 5 == 0 {
    //         println!("buzz");
    //     } else {
    //         println!("{}", count);
    //     }
    //     count += 1;
    // }

    // For Range:

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The values from array : {}", element);
    }

    for range in 1..5 {
        println!("The values from range: {}", range);
    }

    // for x in 0..101 {
    //     if x % 15 == 0 {
    //         println!("fizzbuzz");
    //     } else if x % 3 == 0 {
    //         println!("fizz");
    //     } else if x % 5 == 0 {
    //         println!("buzz");
    //     } else {
    //         println!("{}", x);
    //     }
    // }
}
