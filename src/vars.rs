// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Phil";
    let mut age = 27;
    println!("I am happy sad as long as i am {} ", age);

    age = 28;

    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign muliple vars
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age)
}