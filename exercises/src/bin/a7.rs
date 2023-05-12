// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Red,
    Green,
    Blue,
}

fn display_color(color: Color) {
    match color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
    }
}

fn main() {
    let red = Color::Red;
    display_color(red);
    let blue = Color::Blue;
    display_color(blue);
    let green = Color::Green;
    display_color(green);
}
