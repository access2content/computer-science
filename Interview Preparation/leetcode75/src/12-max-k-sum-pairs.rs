use std::collections::HashMap;

pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut map = HashMap::new();
    let mut counter = 0;

    //  1. Loop through the numbers
    for num in nums {
        //  1. Check if the matching number is in HashMap
        match map.get(&(k - num)) {
            //  If found, reduce the count and increment counter
            Some(&val) => {
                map.entry(k - num).and_modify(|value| {
                    *value -= 1;
                });

                counter += 1;
                match val == 1 {
                    false => {}
                    true => {
                        map.remove(&(k - num));
                    }
                };
            }
            //  If not in HashMap, increment the counter
            None => {
                if num < k {
                    let count = map.entry(num).or_insert(0);
                    *count += 1;
                }
            }
        };
        println!("Count: {counter} | Num: {num} | Map: {:?}", map);
    }

    return counter;
}

fn main() {
    // let input = vec![2, 5, 4, 4, 1, 3, 4, 4, 1, 4, 4, 1, 2, 1, 2, 2, 3, 2, 4, 2];
    // let k = 3;
    let input = vec![4, 4, 1, 3, 1, 3, 2, 2, 5, 5, 1, 5, 2, 1, 2, 3, 5, 4];
    let k = 2;
    let output = max_operations(input, k);
    println!("Output: {output}");
}
