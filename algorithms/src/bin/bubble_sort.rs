use rand::seq::SliceRandom;
use rand::thread_rng;

fn algorithm(arr: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut iteration = 0;

    while iteration != arr.len() - 1 {
        let mut i = 0;
        while i + 1 < arr.len() - iteration {
            if arr[i] > arr[i + 1] {
                let hold = arr[i];
                arr[i] = arr[i + 1];
                arr[i + 1] = hold;
            }
            i += 1;
        }
        iteration += 1;
    }

    return arr;
}

fn main() {
    let mut numbers: Vec<i32> = (0..=10).collect();
    numbers.shuffle(&mut thread_rng());
    println!("INPUT: {:?}", numbers);
    let bubble_sorted = algorithm(&mut numbers);
    println!("Output: {:?}", bubble_sorted);
}
