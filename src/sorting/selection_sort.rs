pub fn selection_sort<T: Ord>(arr: &mut[T]) {
    let len = arr.len();
    for _left in 0..len {
        let mut smallest = _left;
        for _right in (_left + 1) .. len {
            if arr[_right] < arr[smallest] {
                smallest = _right;
            }
        }
        arr.swap(smallest, _left);
    }
}