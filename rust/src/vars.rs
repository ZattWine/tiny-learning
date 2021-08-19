// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Kate";
    let mut age = 23;
    println!("My name is {} and age is {}", name, age);

    age = 24;
    println!("My name is {} and age is {}", name, age);

    // define constant
    const ID: i32 = 001; // i32 = integer 32 bit
    println!("ID: {}", ID);

    // assign multiple vars
    let (my_name, my_age) = ("Kyaw", 24);
    println!("{} is {}", my_name, my_age);
}
