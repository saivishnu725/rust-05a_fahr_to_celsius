use std::io;
fn main() {
    // Convert temperatures between Fahrenheit and Celsius

    // celsius to fahrenheit
    // forumla --> °F = (°C × 9/5) + 32

    // fahrenheit to celsius
    // forumla --> °C = (°F - 32) × 5/9

    // input stuff
    // declare a mutable String variable for degree-type and value
    let mut degree: String = String::new();
    let mut value: String = String::new();
    loop {
        degree.clear(); // read_line() appends to the previous content so clearing the variable

        println!("Enter 'F' Fahrenheit or 'C' for Celsius or 'Q' to quit");
        // keep looping until F or C or Q is entered
        io::stdin() //call the stdin
            .read_line(&mut degree) // read the line and save it in the variable `degree`
            .expect("Failed to get the input!"); // handle error

        // cant compare String and str, so converting to str using `as_str()`
        let degree: &str = degree.as_str().trim(); // trim to remove the newline

        // comparision
        match degree {
            "F" => {
                println!("fahrenheit");
                break;
            }
            "C" => {
                println!("celsius");
                break;
            }
            "Q" => {
                println!("Exiting!!");
                return;
            }
            _ => {
                println!("Invalid input. enter F or C or Q (to quit)");
                continue;
            } // handling invalid inputs
        } // degree
    } // degree loop
    loop {
        // input the value if 'degree' is proper

        value.clear(); // read_line() appends to the previous content so clearing the variable

        println!("Enter the value: ");
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read the input!");
        println!("Value before parsing = {}", value);

        // parse it to a floating type
        let value: f32 = match value.trim().parse() {
            Ok(num) => num,
            Err(a) => {
                println!("Enter a floating value! {a}");
                continue;
            }
        }; // value
        println!("Value after parsing = {value}");

        // break out of the loop when done
        // break;

        // check F or C and convert accordingly
        match degree.as_str().trim() {
            // fahrenheit to celsius
            // forumla --> °C = (°F - 32) × 5/9
            "F" => {
                let result: f32 = (value - 32.0) * (5.0 / 9.0);
                println!("{value}°F in °C = {result}");
                return;
            }
            // celsius to fahrenheit
            // forumla --> °F = (°C × 9/5) + 32
            "C" => {
                let result: f32 = (value * (9.0 / 5.0)) + 32.0;
                println!("{value}°C in °F = {result}");
                return;
            }
            _ => {
                println!("somehow a third value got here. no idea how");
                return;
            }
        }
    } // value loop
}
