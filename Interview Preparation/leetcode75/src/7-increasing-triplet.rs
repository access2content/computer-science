pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let (mut first, mut second) = (i32::MAX, i32::MAX);
    for n in nums {
        println!("First: {first}, Second: {second}");
        if n < first {
            first = n;
            continue;
        }
        if n < second && first < n {
            second = n;
            continue;
        }
        if n > second {
            return true;
        }
    }
    false
}

fn main() {
    let input = vec![5, 6, 1, 7];
    let output = increasing_triplet(input);
    println!("Exists: {}", output);
}
