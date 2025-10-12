use std::io;

fn calculate_discount(bill: f64) -> f64 {
    if bill > 10_000.00 {
        0.15 * bill as f64
    } else if bill > 5_000.00 {
        0.10 * bill as f64
    } else {
        0.00
    }
}


fn main() {
    println!("Welcome to your Discount Calculator.");
    let mut input = String::new();
    println!("Please enter your final bill: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let discount = calculate_discount(input.trim().parse::<f64>().unwrap());
    println!("Your discount is: ₦{:.2}", discount);
}
