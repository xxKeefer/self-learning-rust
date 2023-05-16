#[allow(unused_variables)]
fn algorithm(arr: Vec<i32>, val: i32) -> bool {
    return true;
}

fn main() {
    let numbers: Vec<i32> = (0..=999).collect();
    let res = algorithm(numbers, 10);
    match res {
        true => println!("Number exists"),
        _ => println!("Number not in array."),
    }
}
