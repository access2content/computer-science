pub fn is_subsequence(s: String, t: String) -> bool {
    if s.len() == 0 {
        return true;
    }

    let test: Vec<char> = s.chars().collect();
    let main: Vec<char> = t.chars().collect();

    let first_char = test[0];
    let mut test_index: usize = 0;

    //  Loop over the main string till the first character is found from the test string
    for value in main {
        if value == test[test_index] {
            test_index += 1;
        } else if value == first_char {
            test_index = 1;
        }

        if test_index == test.len() {
            return true;
        }
    }

    return test.len() == test_index;
}

fn main() {
    let input = String::from("bb");
    let test = String::from("ahbgdc");

    let output = is_subsequence(input, test);
    println!("Output: {output}");
}
