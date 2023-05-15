// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    color: String,
}

impl Person {
    fn new(age: i32, name: String, color: String) -> Self {
        Self { age, name, color }
    }

    fn print_info(&self) {
        println!("{}: {}", &self.name, &self.color)
    }
}

fn main() {
    let people = vec![
        Person::new(9, String::from("Dan"), String::from("red")),
        Person::new(10, String::from("Ben"), String::from("blue")),
        Person::new(11, String::from("Sam"), String::from("green")),
        Person::new(12, String::from("Ian"), String::from("yellow")),
    ];

    for p in people {
        if p.age <= 10 {
            p.print_info();
        }
    }
}
