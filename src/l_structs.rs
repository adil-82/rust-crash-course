// Structs - Used to create custom data types

// Traditional
#[derive(Debug)]
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// Tuple Struct:
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct person:
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Method: Get full name:
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name:
    fn set_last_name(&mut self, last: &str ) {
        self.last_name = last.to_string();
    }

    // Name to tuple:
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    println!("**************l_structs.rs***************");

    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };
    let mut c = Color(255, 0, 0);
    // c.red = 200;
    c.1 = 200;
    // println!("Color: {} {} {}", c.red, c.green, c.blue);
    // println!("Color: {:?} ", (c.red, c.green, c.blue));
    println!("Color: {:?}", (c.0, c.1, c.2));

    let mut p = Person::new("John", "Doe");
    println!("Person {} {}", p.first_name, p.last_name);
    // println!("Person {:?}", (p.first_name, p.last_name));
    p.set_last_name("Williams");
    println!("Person: {}", p.full_name());
    println!("Person Tuple: {:?}", p.to_tuple());
}
