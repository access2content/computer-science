pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut current_index: usize = 0;

    for i in 0..nums.len() {
        match nums[i] == 0 {
            true => {}
            false => {
                nums.swap(i, current_index);
                current_index += 1;
            }
        }
    }
}

fn main() {
    let mut input = vec![0, 1, 0, 3, 12];
    // let mut input = vec![2, 1];

    move_zeroes(&mut input);
    println!("Output: {:?}", input);
}
