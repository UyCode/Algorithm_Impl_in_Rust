pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);
        merge(arr, mid);
    }
}

fn merge<T: Ord + Copy>(arr: &mut [T], mid: usize) {
    let left_half = arr[..mid].to_vec();
    let right_half = arr[mid..].to_vec();

    // index to track the position while merging.
    let mut left: usize = 0;
    let mut right: usize = 0;

    for v in arr {
        if right == right_half.len() || (left < left_half.len() && left_half[left] < right_half[right]) {
            *v = left_half[left];
            left += 1;
        } else {
            *v = right_half[right];
            right += 1;
        }
    }
}