use std::collections::HashSet;

pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    //  Create a HashSet from the vectors
    let mut set1: HashSet<i32> = nums1.into_iter().collect();
    let mut set2: HashSet<i32> = nums2.into_iter().collect();

    //  Loop through the second HashSet
    let mut set2final: HashSet<i32> = HashSet::new();
    for num in set2.iter() {
        //  If the element is in the first array, delete it
        match set1.remove(&num) {
            true => {}
            //  If not, add it to the second HashSet
            false => {
                set2final.insert(*num);
            }
        }
    }

    //  Create a Vector from both the HashSets
    let output: Vec<Vec<i32>> = vec![set1.into_iter().collect(), set2final.into_iter().collect()];
    return output;
}

fn test1() {
    let nums1 = vec![1, 2, 3];
    let nums2 = vec![2, 4, 6];

    let output = find_difference(nums1, nums2);
    println!("Output: {:?}", output);
}

fn test2() {
    let nums1 = vec![1, 2, 3, 3];
    let nums2 = vec![1, 1, 2, 2];

    let output = find_difference(nums1, nums2);
    println!("Output: {:?}", output);
}

fn main() {
    test2();
}
