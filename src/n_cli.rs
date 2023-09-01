use std::env;

pub fn run() {
    println!("**************n_cli.rs***************");

    let args: Vec<String> = env::args().collect();

    // println!("Args: {:?}", args);
    // RUN: cargo run hello world => 
    // Args: ["target\\debug\\rust-crash-course.exe", "hello", "world"]

    let command = args[1].clone();
    // println!("Command: {}", command);

    let name = "Adil";
    let status = "100%";

    if command == "hello" {
        println!("Hi {} , how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not valid command!");
    }
}
