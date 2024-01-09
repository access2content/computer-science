fn smallestIndex(input: &[i32], mut start: isize, mut end: isize) -> i32 {
    if start == -1 {
        start = 0;
    }
    if end == -1 {
        end = (input.len() - 1) as isize;
    }

    let mut min_value = input[0];
    let mut min_index: i32 = 0;

    for i in 1..input.len() {
        if input[i] < min_value {
            min_value = input[i];
            min_index = i as i32;
        }
    }

    return min_index;
}

fn selection_sort(input: Vec<i32>) -> Vec<i32> {
    for i in 0..input.len() {
        //  1. Find the smallest Index
        let least = smallestIndex(input, 0, input.len() as isize);

        //  2. Swap least with current index
        let mut temp = input[i];
        input[i] = input[least];
        input[least] = temp;
    }
}

fn main() {
    let mut input = vec![3, 2, 4, 5, 1];
    println!("Input: {:?}", input);

    let output = selection_sort(input);
    println!("Output: {:?}", output);
}
