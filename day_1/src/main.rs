use std::io;

fn main() {
    println!("Hello, world!");
    println!("Enter your age");

    let mut user_input= String::new();

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            match user_input.trim().parse::<i32>() {
                Ok(number) => {
                    // Successfully converted to an integer
                    println!("You entered: {}", number);
                    let result: i32 = calc_age_days(number);
                    println!("{}", result);
                }
                Err(_) => {
                    // Failed to convert to an integer
                    eprintln!("Invalid input. Please enter an integer.");
                }
            }
        }
        Err(error) => {
            // Handle the error (e.g., if the user presses Ctrl+C)
            eprintln!("Error reading input: {}", error);
        }
    }
    
}

fn calc_age_days(age: i32) -> i32 {
    let mut days: i32 = age * 365;
    let leap_year_day = age / 4;
    days = days + leap_year_day;
    return days;
}
