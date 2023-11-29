pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut output: Vec<bool> = Vec::new();

    //  1. Find the max candies among the current ones
    let max = candies.iter().max().unwrap();

    //  2. For each candies, check again for the ones that are >= max candies
    // candies.iter().map(|&c| c + extra_candies >= *max).collect()
    for candy in &candies {
        let result = match candy + extra_candies >= *max {
            true => true,
            false => false,
        };

        output.push(result);
    }

    return output;
}

pub fn main() {
    let candies = vec![2, 3, 5, 1, 3];
    let extra: i32 = 3;
    let output = kids_with_candies(candies, extra);

    println!("Output: {:?}", output);
}
