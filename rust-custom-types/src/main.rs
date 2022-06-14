#![allow(dead_code)]


#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}


enum Status {
    SUCCESS,
    ERROR,
    WARNING,
}


const ZERO: i32 = 0;


fn main() {

    // STRUCT
    let name = String::from("JOHN LOCK");
    let age = 20;
    let john = Person { name, age };

    println!("{:?}", john);

    // ENUM
    use crate::Status::*;

    let status = SUCCESS;
    match status {
        SUCCESS => println!("SUCCESS"),
        ERROR => println!("ERROR"),
        _ => {}
    }

    // CONSTANTS
    println!("ZERO {}", ZERO);
}
