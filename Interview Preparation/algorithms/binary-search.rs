pub fn binary_search(array: &[i32], value: i32, low: usize, high: usize) -> Option<usize> {
    if low > high {
        return None;
    }

    let mid: usize = (low + high) / 2;

    if array[mid as usize] < value {
        return binary_search(array, value, mid + 1, high);
    }

    if array[mid as usize] > value {
        return binary_search(array, value, low, mid - 1);
    }

    if array[mid as usize] == value {
        return Some(mid as usize);
    }

    return None;
}

fn main() {
    let sorted_array: Vec<i32> = vec![1, 2, 3, 4, 5];
    let value = 5;

    let output = binary_search(&sorted_array, value, 0, sorted_array.len() - 1);
    println!("Index: {:?}", output);
}
