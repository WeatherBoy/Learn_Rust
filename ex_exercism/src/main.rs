pub fn reverse(input: &str) -> String {
    let mut string_input = input.to_string();

    let mut reversed_string = String::from("");

    while !string_input.is_empty() {
        reversed_string.push(string_input.pop().unwrap());
    }

    reversed_string
}

fn main() {
    let to_reverse: &str = "Donkey Kong N64";
    let reversed = reverse(to_reverse);
    println!("Reversed '{}': {}", to_reverse, reversed);
}
