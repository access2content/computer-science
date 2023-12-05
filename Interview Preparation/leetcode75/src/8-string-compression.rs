pub fn compress(chars: &mut Vec<char>) -> i32 {
    let mut last_char: char = chars[0];
    let mut last_char_count: usize = 1;
    let mut current_index = 0;

    //  1. Loop through the characters
    for i in 1..chars.len() {
        if chars[i] == last_char {
            last_char_count += 1;
        } else if last_char != chars[i] {
            chars[current_index] = last_char;
            current_index += 1;

            if last_char_count > 1 {
                let value: Vec<char> = last_char_count.to_string().chars().collect();
                for val in value {
                    chars[current_index] = val;
                    current_index += 1;
                }
            }
            last_char_count = 1;
            last_char = chars[i];
        }
    }

    //  2. Process the last pending character
    chars[current_index] = last_char;
    current_index += 1;

    if last_char_count > 1 {
        let value: Vec<char> = last_char_count.to_string().chars().collect();
        for val in value {
            chars[current_index] = val;
            current_index += 1;
        }
    }

    return current_index as i32;
}

fn main() {
    let mut input = vec![
        'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
    ];
    let output = compress(&mut input);

    println!("Output: {}", output);
}
