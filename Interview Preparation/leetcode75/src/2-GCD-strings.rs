pub fn gcd_of_strings(str1: String, str2: String) -> String {
    //  1. Find the smallest of the 2
    let smallest: &str = if str1.len() < str2.len() {
        &str1
    } else {
        &str2
    };

    //  2. Find the first repeating character till the end
    //  If no repeating characters, use the full string
    //  If repeating character found, check if the full string is repeated.
    //      If full is not repeated, need to increase the GCD
    //      If yes, then we can use that as the GCD to check with the 2nd string
    //
    //  2. Find the first repeating character till the end
    let mut smallest_chars = smallest.chars();
    let first_char: char = match smallest_chars.next() {
        Some(value) => value,
        None => char::from('\0'),
    };

    let mut gcd: String = String::from(first_char);

    //  Do this thing till the is GCD is true
    //  2.1. Loop over the characters till we find the first repeat
    loop {
        for value in smallest_chars {
            if value == first_char {
                break;
            } else {
                gcd.push(value);
            }
        }

        println!("GCD: {}", gcd);
        //  2.2. Check if this is a GCD or is there a bigger string
        if is_gcd(&smallest, &gcd) {
            break;
        }
        //  If not GCD, find the bigger string
    }

    println!("Final GCD: {}", gcd);

    return smallest.to_string();
}

fn is_gcd(input: &str, gcd: &str) -> bool {
    let input_chars: Vec<char> = input.chars().collect();
    let gcd_chars: Vec<char> = gcd.chars().collect();

    //  Loop over the input characters
    for i in 0..input_chars.len() {
        println!("{},{}", input_chars[i], gcd_chars[i % gcd_chars.len()]);

        if input_chars[i] != gcd_chars[i % gcd_chars.len()] {
            return false;
        }
    }

    return true;
}

fn main() {
    let input1: String = String::from("ABABABABABAB");
    let input2: String = String::from("ABADABAD");

    let output: String = gcd_of_strings(input1, input2);
    println!("Output: {}", output);
}
