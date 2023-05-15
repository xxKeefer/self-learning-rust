// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum ContainerColor {
    Red,
}

impl ContainerColor {
    fn print(&self) {
        match self {
            ContainerColor::Red => println!("red"),
        }
    }
}

struct Dimensions {
    height: f64,
    width: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!(
            "{} units x {} units x {} units",
            self.height, self.width, self.depth
        )
    }
}

struct Container {
    weight: f64,
    dimensions: Dimensions,
    color: ContainerColor,
}

impl Container {
    fn new(dimensions: Dimensions, weight: f64, color: ContainerColor) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print_info(&self) {
        println!("BOX WEIGHT:");
        println!("{}", self.weight);
        println!("BOX DIMENSIONS:");
        self.dimensions.print();
        println!("BOX COLOUR:");
        self.color.print();
    }
}

fn main() {
    let size = Dimensions {
        height: 1.2,
        width: 3.4,
        depth: 5.6,
    };
    let container = Container::new(size, 15.5, ContainerColor::Red);
    container.print_info()
}
