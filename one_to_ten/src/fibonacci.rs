use std::io;
use crate::get_input;

pub fn fibonacci_fn(){
    println!("Enter the max number of fibonacci to print");

    let ceil_input = get_input!();
    let mut normal_prog_flow = true;
    let mut ceil_val:usize = 0;

    match ceil_input {
        Some(val) => {
            match val.parse::<usize>() {
                Ok(parsed_val) => {
                    ceil_val = parsed_val;
                },
                Err(err) => {
                    println!("Invalid digit value. Error: {}", err);
                    normal_prog_flow = false;
                }
            };
        },
        None => {
            normal_prog_flow = false;
        },
    };

    if normal_prog_flow {
        fibonacci_generator(ceil_val);
    }
}

fn fibonacci_generator(max_val: usize){
    let mut sequence:Vec<usize> = vec![0];
    let mut counter = 0;
    let mut current_value: usize = 1;

    loop {
        if current_value >= max_val {
            break;
        }
        sequence.push(current_value);
        current_value = sequence[counter + 1] + sequence[counter];
        counter += 1;
    }

    println!("The fibonacci sequnce is:");

    for value in &sequence {
        if sequence.ends_with(&[*value]) {
            println!("{}", value);
        } else {
            print!("{}, ", value);
        }
    }
}