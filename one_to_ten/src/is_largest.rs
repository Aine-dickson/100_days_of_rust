use crate::get_input;
use std::io;

pub fn is_largest_fn(){
    println!("Enter list of comma separated numbers e.g 1, 3, 9");

    let list_of_numbers = get_input!();
    let mut actual_list:Vec<f32> = Vec::new();
    let mut normal_flow = true;

    match list_of_numbers {
        Some(list) => {
            let list_values:Vec<&str> = list.split(",").collect();
            for item in list_values {
                match item.trim().parse::<f32>() {
                    Ok(value) => {
                        actual_list.push(value);
                    },
                    Err(_) => {
                        println!("List contains non digit value");
                        normal_flow = false;
                        break;
                    },
                }
            }
        },
        None => {
            normal_flow = false;
        },
    }

    if normal_flow {
        let mut largest = actual_list[0];

        for number in &actual_list {
            if *number > largest {
                largest = *number;
            }
        }

        println!("Of {:?}, largest is: {}", actual_list, largest);
    }
}