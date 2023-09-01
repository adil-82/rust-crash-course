// Enums are types which have a few definite values

enum Movement {
    // Variants:
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // Perform action depending on info
    match m {
        Movement::Up => println!("Avatar moving Up"),
        Movement::Down => println!("Avatar moving Down"),
        Movement::Left => println!("Avatar moving Left"),
        Movement::Right => println!("Avatar moving Right"),
    }
}

pub fn run() {
    println!("**************l_structs.rs***************");

    let avatar1 = Movement::Right;
    let avatar2 = Movement::Left;
    let avatar3 = Movement::Down;
    let avatar4 = Movement::Up;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
