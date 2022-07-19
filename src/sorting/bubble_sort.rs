pub fn bubble_sort<T: Ord>(arr: &mut[T]) {
    let len = arr.len();
    for _left in 0..len {
        for _right in (_left+1)..len {
            if arr[_left] > arr[_right] {
                arr.swap(_left, _right);
            }
        }
    }
}