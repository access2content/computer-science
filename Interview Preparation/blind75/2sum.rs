use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = HashMap::new();

    for i in 0..nums.len() {
        match result.get(&(target - nums[i])) {
            Some(&index) => return vec![index, i as i32],
            None => {result.insert(nums[i], i as i32);},
        }
    }

    vec![]
}

fn main(){
    let input = vec![1, 2, 5, 4];

    let output = two_sum(input, 5);
    println!("{:?}", output);
}