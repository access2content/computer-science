pub fn close_strings(word1: String, word2: String) -> bool {
    //  Should have the same length
    if word1.len() != word2.len() {
        return false;
    }

    let mut set1: [bool; 26] = [false; 26];
    let mut val1: [usize; 26] = [0; 26];
    for val in word1.chars() {
        let char_num = val as usize - 'a' as usize;
        val1[char_num] += 1;
        set1[char_num] = true;
    }

    let mut set2: [bool; 26] = [false; 26];
    let mut val2: [usize; 26] = [0; 26];
    for val in word2.chars() {
        let char_num = val as usize - 'a' as usize;
        val2[char_num] += 1;
        set2[char_num] = true;
    }
    val1.sort();
    val2.sort();

    for i in 0..26 {
        if set1[i] != set2[i] {
            return false;
        }
        if val1[i] != val2[i] {
            return false;
        }
    }

    true
}

fn test1() {
    let output = close_strings("vivek".to_string(), "keviv".to_string());
    println!("Output: {output}");
}

fn main() {
    test1();
}
