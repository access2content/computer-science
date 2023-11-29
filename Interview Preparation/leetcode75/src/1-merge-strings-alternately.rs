fn merge_alternately(word1: String, word2: String) -> String {
    let mut output = "".to_string();

    let mut word1chars = word1.chars();
    let mut word2chars = word2.chars();

    loop {
        match (word1chars.next(), word2chars.next()) {
            (Some(c1), Some(c2)) => {
                output.push(c1);
                output.push(c2);
            }
            (Some(c1), None) => output.push(c1),
            (None, Some(c2)) => output.push(c2),
            (None, None) => break,
        }
    }

    output
}

fn main() {
    let str1 = String::from("Vivek");
    let str2 = String::from("Agrawal");
    let output = merge_alternately(str1, str2);
    println!("Output: {}", output);
}
