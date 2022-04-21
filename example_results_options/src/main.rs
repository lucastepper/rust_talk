#![allow(unused)]
use std::fs::File;


fn main() {
    // Options handle if a value can be set or None.
    let maybe_number: Option<i32> = Some(5);
    // get the value inside the option or crash the programm if None
    let number = maybe_number.unwrap();

    // handle the case when maybe_number is None
    let maybe_number: Option<i32> = None;
    let number = match maybe_number {
        Some(number) => number,
        None => {
            println!("maybe_number was None, we set it to 0");
            0
        },
    };

    // Enums a special type of structs that can have multiple variants.
    enum WeAreCool {
        Yes(String),
        YEEES,
    }
    let are_we_cool = WeAreCool::Yes("We are cool".to_string());
    let are_we_cool = WeAreCool::YEEES;

    // Most important enum: Result
    // Everyting that can fail in rust returns a Result.
    let mut file_creation = File::create("/DoesNotExist");
    match file_creation {
        Ok(file) => {
            // do something with the file
        },
        Err(error) => {
            println!("Error, the file could not be created");
        },
    }

    // For both, if you do not want to deal with them, just use .unwrap()
}
