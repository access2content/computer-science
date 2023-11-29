// pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
//     let mut bed = flowerbed.iter();
//     let mut count: i32 = 0;
//     let mut prev: Option<&i32> = None;
//     let mut current: Option<&i32> = bed.next();
//     let mut next: Option<&i32> = bed.next();
//
//     while current != None {
//         let can_plant: bool = match (prev, current, next) {
//             (None, Some(0), None) => true,
//             (None, Some(0), Some(0)) => true,
//             (Some(0), Some(0), Some(0)) => true,
//             (Some(0), Some(0), None) => true,
//             _ => false,
//         };
//
//         if can_plant {
//             current = Some(&1);
//             count += 1;
//         }
//         if count >= n {
//             return true;
//         }
//
//         prev = current;
//         current = next;
//         next = bed.next();
//     }
//
//     return false;
// }

use std::iter;

pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    n == 0
        || flowerbed
            .iter()
            .chain(iter::once(&0))
            .fold((1, 0), |(zero_count, total), bed| match (zero_count, bed) {
                (_, 1) => (0, total),
                (2, 0) => (1, total + 1),
                _ => (zero_count + 1, total),
            })
            .1
            >= n
}

pub fn main() {
    let bed = vec![0];
    let value = 1;

    let output = can_place_flowers(bed, value);
    println!("Output: {}", output);
}
