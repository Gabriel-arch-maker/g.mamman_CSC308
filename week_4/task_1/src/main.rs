fn get_last_word(full: String) -> String {
    let words: Vec<&str> = full.split(" ").collect();
    words.last().unwrap_or(&"").to_string()
}

fn main() {
    let last_word = get_last_word("Hello world".to_string());
    println!("{}", last_word);
}
