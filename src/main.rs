fn get_string_length(s: String) -> usize {
    s.chars().count()
}

fn main() {
    let my_string = String::from("Hello World!");
    let length = get_string_length(my_string);
    println!("The length of the text: {}", length);
}