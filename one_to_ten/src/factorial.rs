use crate::get_input;
use std::io;

pub fn factorial_fn(){
    println!("Enter test number");

    let test_input = get_input!();
    let mut test_val = 0;
    let mut normal_flow = true;

    match test_input {
        Some(val) => {
            match val.parse::<usize>() {
                Ok(parsed_val) => {
                    test_val = parsed_val;
                },
                Err(err) => {
                    normal_flow = false;
                    println!("Error: {}", err);
                },
            }
        },
        None => {
            normal_flow = false;
        }
    }

    if normal_flow {
        let mut current_val = test_val;
        let mut result = 1;
        let mut processed_values: Vec<usize> = Vec::new();

        loop {
            if current_val < 1 {
                break;
            }

            result *= current_val;
            processed_values.push(current_val);
            current_val -= 1;
        }

        println!("{}! = {}\nWorking:",test_val, result);

        for value in &processed_values {
            if processed_values.ends_with(&[*value]) {
                println!("{}", value)
            } else {
                print!("{} x ", value)
            }
        }
    }
}