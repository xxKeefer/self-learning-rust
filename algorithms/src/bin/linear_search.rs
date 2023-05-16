fn linear_search(arr: Vec<i32>, val: i32) -> Option<i32> {
    for x in arr {
        if x == val {
            return Some(x);
        }
    }
    return None;
}

fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let res = linear_search(numbers, 10);
    match res {
        Some(_) => println!("Number exists"),
        _ => println!("Number not in array."),
    }
}
