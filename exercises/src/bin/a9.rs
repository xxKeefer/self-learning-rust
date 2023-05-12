// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn make_coord(x: i32, y: i32) -> (i32, i32) {
    (x, y)
}

fn main() {
    let (_, y) = make_coord(99, 5);

    if y > 5 {
        println!("y is >5");
    } else if y < 5 {
        println!("y is <5");
    } else {
        println!("y is =5");
    }
}
