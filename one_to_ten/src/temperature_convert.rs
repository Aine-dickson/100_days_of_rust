use crate::get_input;
use std::io;

pub fn temp_convert_fn(){
    println!(r#"
        Choose action to continue
        1. Convert from Celsius
        2. Convert from Fahrenheit
    "#);

    let choice = get_input!();

    match choice {
        Some(choice) => {
            match choice.as_str() {
                "1" => {
                    to_fahrenheit()
                },
                "2" => {
                    to_celsius();
                },
                _ => {println!("Invalid choice")}
            }
        },
        None => {},
    }
}

fn to_celsius(){
    let mut normal_flow = true;
    println!("Enter fahrenheit value");

    let fah_val = get_input!();
    let mut actual_val: f32 = 0.0;

    match fah_val {
        Some(value) => {
            match value.parse::<f32>() {
                Ok(value) => actual_val = value,
                Err(err) => {
                    println!("{}", err);
                    normal_flow = false;
                },
            }
        },
        None => {
            normal_flow = false;
        },
    }

    if normal_flow {
        let result = (actual_val - 32.0) * (5f32/9f32);
        println!("{}°F = {}°C", actual_val, result);
    }
}

fn to_fahrenheit(){
    let mut normal_flow = true;
    println!("Enter celsius value");

    let cel_val = get_input!();
    let mut actual_val: f32 = 0.0;

    match cel_val {
        Some(value) => {
            match value.parse::<f32>() {
                Ok(value) => actual_val = value,
                Err(err) => {
                    println!("{}", err);
                    normal_flow = false;
                },
            }
        },
        None => {
            normal_flow = false;
        },
    }

    if normal_flow {
        let result = ((9f32/5f32) * actual_val) + 32.0;
        println!("{}°F = {}°C", actual_val, result);
    }
}