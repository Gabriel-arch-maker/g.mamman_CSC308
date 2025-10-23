fn main () {
    let mut message = String::from("Hello");
    show_message(&message);
    add_note(&mut message);
    println!("Final message: {}", message);
}
fn show_message (message: &String) {
    println!("Current message: {}", message);
}

fn add_note(message: &mut String) { message.push_str(", world!");
}
