pub fn insertion_sort<T> (arr: &mut[T])where T: PartialOrd +  Copy{
    let n = arr.len();
    for i in 1..n {
        let current = arr[i];
        let mut j = i;
        while  arr[j-1] > current {
            arr.swap(j, j-1);
            if j == 1 { break; }
            j -= 1;
        }
    }
}
