use std::io;

fn convert_celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0/5.0) + 32.0
}

fn main() {
    println!("Welcome to the Celsius to Fahrenheit converter.");
    let mut input = String::new();
    println!("Enter temperature in Celsius: ");
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let fahrenheit: f64 = convert_celsius_to_fahrenheit(input.trim().parse::<f64>().unwrap());
    println!("Temperature in Fahrenheit: {:.2}ºF", fahrenheit);
}
