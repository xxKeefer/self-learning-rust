// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {
    let first_name = "Daniel";
    let last_name = "Keefer";

    fn display_string(string: &str) {
        println!("{}", string);
    }

    display_string(first_name);
    display_string(last_name);

    println!("My name is {} {}.", first_name, last_name);
}
