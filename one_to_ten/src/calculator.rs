use crate::get_input;
use std::io;

macro_rules! calculate {
    ($expression: ident) => {
        {
            let mut operation = None;
            let mut normal_flow = true;
            let mut actual_operation = String::new();
            let mut result = 0;

            
            for c in $expression.chars() {
                match c {
                    '+' | '-' | '/' | '*' => {
                        operation = Some(c);
                        break;
                    }
                    _ => {
                        continue
                    }
                }
            }
            
            match operation {
                Some(oper) => {
                    actual_operation.push(oper);
                },
                None => {
                    println!("No operation identified");
                    normal_flow = false;
                }
            }
            
            if normal_flow {
                let split_expr:Vec<&str> = $expression.split(actual_operation.as_str()).collect();
                let lefthand = split_expr[0].trim().parse::<usize>();
                let righthand = split_expr[1].trim().parse::<usize>();

                match (lefthand.clone(), righthand.clone()) {
                    (Ok(_), Ok(_)) => {
                        normal_flow = true;
                    },
                    _ => {
                        normal_flow = false;
                        println!("Error: Found invalid integer");
                    }
                }

                if normal_flow {
                    match actual_operation.as_str() {
                        "+" => {result = lefthand.ok().unwrap() + righthand.ok().unwrap();},
                        "-" => {result = lefthand.ok().unwrap() - righthand.ok().unwrap();},
                        "*" => {result = lefthand.ok().unwrap() * righthand.ok().unwrap();},
                        "/" => {result = lefthand.ok().unwrap() / righthand.ok().unwrap();},
                        _ => {println!("Internal cal error")}
                    }
                }
                
            }
            

            result
        }
    };
}

pub fn calc() {
    println!("Enter expression, e.g 2+2");

    let expression = get_input!();


    match expression {
        Some(expr) => {
            let result = calculate!(expr);
            println!("{:?}", result);
        },
        None => todo!(),
    }
}
