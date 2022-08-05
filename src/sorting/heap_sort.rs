pub fn heap_sort<T: Ord + Copy>(arr: &mut [T]) {
    for i in (1..=((arr.len() / 2) + 1)).rev() {
        adjustment(arr, i, arr.len())
    }

    for i in (1..=(arr.len() - 1)).rev() {
        let tmp: T = arr[0];
        arr[0] = arr[i];
        arr[i] = tmp;
        adjustment(arr, 0, i);
    }
}

fn adjustment<T: Ord + Copy>(arr: &mut [T], position: usize, length: usize) {
    let mut child: usize = 2 * position + 1;
    if child + 1 < length && arr[child] > arr[child + 1] {
        child += 1;
    }

    if child < length && arr[position] > arr[child] {
        let tmp: T = arr[position];
        arr[position] = arr[child];
        arr[child] = tmp;
        adjustment(arr, child, length);
    }
}