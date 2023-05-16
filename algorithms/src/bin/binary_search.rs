// Binary Search - returns an Option of the index at which the target exists in the Vec
fn algorithm(arr: &Vec<i32>, target: &i32) -> Option<usize> {
    let mut low: usize = 0;
    let mut high: usize = arr.len();

    while low < high {
        let middle = ((high - low) / 2) + low;
        let inspect = &arr[middle];

        if inspect == target {
            return Some(middle);
        }

        if inspect < target {
            low = middle + 1
        } else {
            high = middle - 1
        }
    }
    return None;
}

fn main() {
    let numbers: Vec<i32> = (999..=9999).collect();
    let res = algorithm(&numbers, &99999);
    match res {
        Some(index) => println!("Number exists at index: {} ", index,),
        _ => println!("Number not in array."),
    }
}
