use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "user";
    let status = "100%";

    if command == "greet" {
        println!("Hi, {}, nice to meet you.", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("Invalid command!");
    }

    // println!("Command: {}", command);
}
