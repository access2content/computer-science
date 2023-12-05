pub fn reverse_vowels(s: String) -> String {
    //  1. Convert to a vector of characters
    let mut input: Vec<char> = s.chars().collect();

    //  2. Keep two pointers to find first and last vowel
    let mut first: usize = 0;
    let mut last: usize = input.len() - 1;

    while first < last {
        //  Get a vowel in forward direction
        for i in first..last {
            match input[i as usize].to_ascii_lowercase() {
                'a' | 'e' | 'i' | 'o' | 'u' => break,
                _ => first += 1,
            }
        }

        for i in (first..last + 1).rev() {
            match input[i as usize].to_ascii_lowercase() {
                'a' | 'e' | 'i' | 'o' | 'u' => break,
                _ => last -= 1,
            }
        }

        if first > last {
            break;
        }

        //  Swap the characters
        let temp = input[first];
        input[first] = input[last];
        input[last] = temp;

        //  Increment the first and decrement the last
        if first == input.len() {
            break;
        }
        if last == 0 {
            break;
        }
        first += 1;
        last -= 1;
    }

    input.into_iter().collect()
}

fn main() {
    let input = "a.".to_string();
    let output = reverse_vowels(input);
    println!("Output: {}", output);
}
