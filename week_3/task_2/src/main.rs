fn main() {
    let mut name = String::from("Ada");
    print_name(&name);
    append_title(&mut name);
    println!("Final name: {}", name );
}

fn print_name(n: &String) {
    println!("Name: {}", n);
}
fn append_title(name: &mut String) {
    name.push_str(" Lovelace")
}