use std::io;

// Function to calculate the electricity bill
fn calculate_bill(consumption: f64) -> f64 {
    let rate = if consumption > 200.0 {
        30.0
    } else if consumption > 100.0 {
        25.0
    } else {
        20.0
    };

    consumption * rate
}

fn main() {
    println!("Enter your electricity consumption in kWh:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let consumption: f64 = input.trim().parse().expect("Please enter a valid number");

    let total_bill = calculate_bill(consumption);

    println!("Your total electricity bill is: ₦{:.2}", total_bill);
}
