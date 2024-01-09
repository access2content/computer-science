pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut answer = vec![1; nums.len()];

    //  Loop over the nums once to calculate prefix product
    for i in 1..nums.len() {
        answer[i] = answer[i - 1] * nums[i - 1];
    }

    //  Loop in the reverse direction
    let mut temp = 1;
    for i in (0..nums.len() - 1).rev() {
        temp *= nums[i + 1];
        answer[i] *= temp;
    }

    return answer;
}

fn main() {
    let input: Vec<i32> = vec![1, 2, 3, 4];
    println!("Input: {:?}", input);
    let output = product_except_self(input);
    println!("Output: {:?}", output);
}
