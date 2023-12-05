// Given an input string s, reverse the order of the words.
//
// A word is defined as a sequence of non-space characters. The words in s will be separated by at least one space.
//
// Return a string of the words in reverse order concatenated by a single space.
//
// Note that s may contain leading or trailing spaces or multiple spaces between two words. The returned string should only have a single space separating the words. Do not include any extra spaces.

pub fn reverse_words(s: String) -> String {
    let mut output = "".to_string();

    //  Convert the input string to a character array
    let input: Vec<char> = s.chars().collect();

    // Loop over the string in reverse order
    let mut word = "".to_string();
    for i in (0..input.len()).rev() {
        match input[i] {
            ' ' => {
                //  Check if the word has at least 1 char. Then, push it to output and clear it
                if !word.is_empty() {
                    if !output.is_empty() {
                        output.push(' ');
                    }
                }

                output.push_str(&word.chars().rev().collect::<String>());
                word.clear();
            }
            _ => {
                //  If character, push it in the text
                word.push(input[i]);
            }
        }
    }

    //  Add the last word
    if !word.is_empty() {
        if !output.is_empty() {
            output.push(' ');
        }

        output.push_str(&word.chars().rev().collect::<String>());
    }

    return output;
}

fn main() {
    let input = String::from("This is a String");
    let output = reverse_words(input);
    println!("Output: {output}");
}
