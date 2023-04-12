/*
    Reverse a string.
        Thats it. Reverse a string. Do it!
*/
pub fn reverse(input: &str) -> String {
    // chars() - makes an iterator of chars
    // rev() - reverses the iterator
    // collect() - collects it to a string
    input.chars().rev().collect()
}

fn main() {
    // Testing whether the reverse assignment was done properly.
    let to_reverse: &str = "Donkey Kong N64";
    let reversed = reverse(to_reverse);
    println!("Reversed '{}': {}", to_reverse, reversed);
}
