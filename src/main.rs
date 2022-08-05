pub mod sorting;

#[cfg(test)]
mod tests {
    use sorting;

    use super::*;

    use self::rand::Rng;

    extern crate rand;

    /**
     * selection sort is a sorting algorithm that works by iterating over the array,
     * finding the smallest element in the array and swapping it with the first element.
     * It then repeats this process for the next element in the array.
     * This is done until the array is sorted.
     * Time complexity: O(n^2)
     * Space complexity: O(1)
     * Stable: Yes
     * In-place: Yes
     * Online: Yes
     */
    #[test]
    fn selection_sort() {
        let mut res = vec![];
        let mut rng = rand::thread_rng();
        for _i in 0..10000 {
            res.push(rng.gen_range(0..10000));
        }
        sorting::selection_sort(&mut res);
        for i in 0..res.len() - 1 {
            assert!(res[i] <= res[i + 1]);
        }
    }

    /**
    * Bubble sort is a simple sorting algorithm that works by repeatedly swapping adjacent elements if they are in wrong order.
     * Time Complexity: O(n^2)
     * Space Complexity: O(1)
     * Stable: Yes
     * In-place: Yes
     * Online: Yes
    */
    #[test]
    fn bubble_sort() {
        let mut res = vec![];
        let mut rng = rand::thread_rng();
        for _i in 0..10000 {
            res.push(rng.gen_range(0..10000));
        }
        sorting::bubble_sort(&mut res);
        for i in 0..res.len() - 1 {
            assert!(res[i] <= res[i + 1]);
        }
    }

    /**
     * Quick sort is a divide and conquer algorithm.
     * It works by recursively splitting the array in two parts,
     * and sorting each part.
     * Time Complexity: O(n log n)
     * Space Complexity: O(log n)
     * Stable: Yes
     * In-place: Yes
     * Online: Yes
     */

    #[test]
    fn quick_sort() {
        let mut res = vec![];
        let mut rng = rand::thread_rng();
        for _i in 0..10000 {
            res.push(rng.gen_range(0..10000));
        }

        let len = res.len();
        let mut result = vec![0; len];
        for i in 0..res.len() {
            result[i] = sorting::quick_sort(&mut res, 0, len - 1, i);
        }
        for i in 0..res.len() - 1 {
            print!("{} ", result[i]);
            if i % 1000 == 0 { println!(); }
        }
    }

    /**
     * merge sort is a recursive algorithm for sorting.
     * It works by recursively splitting the array in two parts,
     * and sorting each part.
     * Time Complexity: O(n log n)
     * Space Complexity: O(log n)
     * Stable: Yes
     * In-place: Yes
     * Online: Yes
     */
    #[test]
    fn merge_sort() {
        let mut res = vec![];
        let mut rng = rand::thread_rng();
        for _i in 0..10000 {
            res.push(rng.gen_range(0..10000));
        }

        sorting::merge_sort(&mut res);
        for _i in 0..res.len() - 1 {
            print!("{} ", res[_i]);
            if _i % 100 == 0 { println!(); }
        }
    }

    /**
     * Heap sort is a recursive algorithm for sorting.
     * It works by recursively splitting the array in two parts,
     * and sorting each part.
     * Time Complexity: O(n log n)
     * Space Complexity: O(log n)
     * Stable: Yes
     * In-place: Yes
     * Online: Yes
     */
    #[test]
    fn heap_sort() {
        let mut res = vec![];
        let mut rng = rand::thread_rng();
        for _i in 0..10000 {
            res.push(rng.gen_range(0..10000));
        }
        sorting::heap_sort(&mut res);
        println!("last one : {} ", res[res.len() - 1]);
        for i in (0..=res.len() - 1).rev() {
            print!("{} ", res[i]);
            if i % 100 == 0 { println!(); }
        }
    }
}

fn main() {
    println!("Starting test....");
}
