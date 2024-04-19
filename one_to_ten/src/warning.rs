#[macro_export] macro_rules! warn {
    () => {
        println!(r#"
            Expected one of arguments below
    
            `hello_world`       to run hello world program
            `fibonacci`         to print a fibonacci sequence
            `reverse_string`    to reverse a given string
            `calculator`        to perform simple caculation
            `temp_convertor`    to convert between Celsius and Fahranheit
            `find_largest`      to determine largest number from a given list
        "#)
    }
}