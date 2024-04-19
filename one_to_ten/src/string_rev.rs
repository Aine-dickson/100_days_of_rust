use crate::get_input;
use std::io;

pub fn string_rev_fn(){
    println!("Enter test number");

    let test_input = get_input!();

    match test_input {
        Some(val) => {
            reverse_string(val)
        },
        None => {},
    }
}

fn reverse_string(string: String){
    let split_string:Vec<&str> = string.split("").collect();
    let mut reversed_string = String::new();
    let mut counter = split_string.len();

    loop {
        if counter < 1 {
            break;    
        }

        let parsed_char = split_string[counter-1].parse::<char>();

        match parsed_char {
            Ok(_char) => {
                reversed_string.push(_char);
            },
            Err(_) => {},
        }

        counter -= 1;
    }

    println!("Reversed {} is: {}", &string, reversed_string);
}