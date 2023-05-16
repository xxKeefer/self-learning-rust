use rand::seq::SliceRandom;
use rand::thread_rng;

fn algorithm(arr: &mut Vec<i32>, iteration: Option<usize>) -> &mut Vec<i32> {
    let recursion = match iteration {
        Some(x) => x,
        _ => 0,
    };

    let mut i = 0;
    while i + 1 < arr.len() - recursion {
        if arr[i] > arr[i + 1] {
            let hold = arr[i];
            arr[i] = arr[i + 1];
            arr[i + 1] = hold;
        }
        i += 1;
    }

    if recursion == arr.len() - 1 {
        return arr;
    } else {
        algorithm(arr, Some(recursion + 1))
    }
}

fn main() {
    let mut numbers: Vec<i32> = (0..=10).collect();
    numbers.shuffle(&mut thread_rng());
    println!("INPUT: {:?}", numbers);
    let bubble_sorted = algorithm(&mut numbers, None);
    println!("Output: {:?}", bubble_sorted);
}
