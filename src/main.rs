pub mod sorting;

#[cfg(test)]
mod tests {
    use sorting;
    //use crate::sorting::selection_sort::selection_sort;
    use super::*;
    extern crate rand;
    use self::rand::Rng;

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
}

fn main() {
    let a = "Hello, world!".to_string();
    println!("{}", a);
    println!("Hello, world!");
}