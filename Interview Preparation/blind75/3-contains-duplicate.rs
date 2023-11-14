use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut duplicates = HashSet::new();

    for num in nums {
        match duplicates.get(&num) {
            Some(&_) => return true,
            None => {
                duplicates.insert(num);
            }
        }
    }

    return false;
}

fn main() {
    let input: Vec<i32> = vec![1, 2, 3, 4, 4];
    let output = contains_duplicate(input);
    println!("Has duplicates? {}", output);
}
