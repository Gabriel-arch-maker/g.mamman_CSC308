fn main() {
    let  name = String::from("Firstname");
    let returned_name = add_surname_to_firstname(name);
    println!("{}", returned_name);

}

fn add_surname_to_firstname(mut name: String) -> String {
    name.push_str("Lastname");
    name
}
