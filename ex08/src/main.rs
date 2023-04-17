const TOO_BIG: usize = 10;
fn main() {
    /*
    Exercise 8.1:
        Given a list of integers, use a vector and return the median (when sorted,
        the value in the middle position) and mode (the value that occurs most often;
        a hash map will be helpful here) of the list.
    */
    println!("\n\n*** Ex 8.1 ***");
    let list_of_ints = [5, 1, 3, 6, 4, 2];
    let mut median_vec = Vec::from(list_of_ints);

    median_vec.sort(); // <-- This just sorts the vector (and it does it on <self>, so no need to reassign).

    if median_vec.len() < TOO_BIG {
        println!("This is the vector sorted!");

        for integer in &median_vec {
            println!("{integer}");
        }
    } else {
        println!("Vector was too big and therefore not printed..")
    }

    // plainly obvious why I HAD to declare this variable.
    let half = median_vec.len() / 2;

    let median: f32 = if median_vec.len() % 2 == 0 {
        (&median_vec[half - 1] + &median_vec[half]) as f32 / 2.0
    } else {
        median_vec[half] as f32
    };

    println!("\nThis is the median of the vector {median}");

    /*
    Exercise 8.2:
        Convert strings to pig latin. The first consonant of each word is moved to
        the end of the word and “ay” is added, so “first” becomes “irst-fay.”
        Words that start with a vowel have “hay” added to the end instead
        (“apple” becomes “apple-hay”).
        Keep in mind the details about UTF-8 encoding!
    */

    let mut test_word = String::from("Fewer");

    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut starts_with_vowel = false;

    for vowel in vowels {
        if test_word.starts_with(vowel) {
            starts_with_vowel = true;
            break;
        }
    }

    /*
    let pig_latin: String = if starts_with_vowel {
        let first_letter = test_word.chars()
    }
    */
}
