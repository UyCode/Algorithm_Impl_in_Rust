pub fn quick_sort(arr: &mut[i32], left: usize, right: usize, index: usize) -> i32 {
    if left == right {
        return arr[left];
    }
    let mut pivot_index = ((left + right) / 2) + 1;
    pivot_index = partition(arr, left, right, pivot_index);


    // match pattern expression for this return statement.
    match pivot_index {
        p_index if index == p_index => arr[index],
        p_index if index < p_index => quick_sort(arr, left, pivot_index - 1, index),
        _ => quick_sort(arr, pivot_index + 1, right, index),
    }
 }

fn partition(arr: &mut[i32], left: usize, right: usize, pivot_index: usize) -> usize {
    let pivot_value = arr[pivot_index];
    arr.swap(pivot_index, right); // move pivot to end
    let mut store_index = left;
    for i in left..(right + 1) {
        if arr[i] < pivot_value {
            arr.swap(store_index, i);
            store_index += 1;
        }
        arr.swap(right, store_index);
    }
    store_index
}