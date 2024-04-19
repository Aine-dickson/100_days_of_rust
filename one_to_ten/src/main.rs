pub mod hello;
pub mod fibonacci;
pub mod factorial;
pub mod scan;
pub mod string_rev;
pub mod calculator;
pub mod temperature_convert;
pub mod warning;
pub mod is_largest;

use std::env::args;
use factorial::factorial_fn;
use fibonacci::fibonacci_fn;
use hello::hello_world;
use string_rev::string_rev_fn;
use calculator::calc;
use temperature_convert::temp_convert_fn;
use is_largest::is_largest_fn;

fn main() {
    let _args = args();

    let challenges: Vec<(String, fn())> = vec![
        (String::from("hello_world"), hello_world),
        (String::from("fibonacci"), fibonacci_fn),
        (String::from("factorial"), factorial_fn),
        (String::from("reverse_string"), string_rev_fn),
        (String::from("calculator"), calc),
        (String::from("temp_convertor"), temp_convert_fn),
        (String::from("find_largest"), is_largest_fn),
    ];

    let mut found = false;

    if args().len() < 2 {
        warn!()
    } else {
        for arg in _args {
            if found {
                break;
            }
            for challenge in &challenges {
                if arg.to_lowercase().cmp(&challenge.0).is_eq() {
                    found = true;
                    challenge.1();
                    break;
                }
            }
        }
    
        if !found {
            println!("Unknown argument");
            warn!()
        }
    }

}
