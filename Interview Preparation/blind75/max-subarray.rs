pub fn simple(nums: Vec<i32>) -> i32 {
    let mut max = nums[0];
    for i in 0..nums.len() {
        for j in i..nums.len() {
            let mut sum = 0;
            for k in i..j + 1 {
                sum += nums[k];
                print!("{}, ", nums[k]);
            }

            if sum > max {
                max = sum;
            }
            print!("= {}", sum);
            println!("");
        }
    }

    return max;
}

fn kadane(inputs: Vec<i32>) -> i32 {
    //  Loop through the input
    let mut local_max = 0;
    let mut max_global = i32::MIN;

    for input in inputs {
        local_max = std::cmp::max(input, local_max + input);
        if local_max > max_global {
            max_global = local_max;
        }
        print!("{}, ", local_max);
    }

    return max_global;
}

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Input: {:?}", nums);
    let output = kadane(nums);
    println!("");
    println!("Output: {}", output);
}
