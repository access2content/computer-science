pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_total: i32 = 0;
    let mut start: usize = 0;
    let mut end: usize = height.len() - 1;

    //  Loop through all the heights
    while start < end {
        //  1. Find the smallest among the two heights
        let min = std::cmp::min(height[start], height[end]);

        //  2. Calculate the current area
        let area = (end as i32 - start as i32) * min;

        //  3. If new area is greater, then update the highest area
        match max_total < area {
            true => {
                max_total = area;
            }
            false => {}
        }

        //  4. Update the start and end
        match height[start] < height[end] {
            true => {
                start += 1;
            }
            false => {
                end -= 1;
            }
        }
    }

    return max_total;
}

fn main() {
    let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let output = max_area(input);
    println!("Output: {output}");
}
