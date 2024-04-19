#[macro_export] macro_rules! get_input {
    () => {
        {
            let mut input = String::new();
            let line = io::stdin().read_line(&mut input);

            let result: Option<String> = match line {
                Ok(_) => Some(input.trim().to_string()),
                Err(err) => {
                    println!("Unable to read input, Error: {}", err);
                    None
                }
            };

            result
        }
    };
}