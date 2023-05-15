// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker_no: Option<i32>,
}

impl Student {
    fn print_locker(&self) {
        match &self.locker_no {
            Some(locker) => println!("{}'s locker number is: {}", self.name, locker),
            None => println!("{} has no locker number assigned,", self.name),
        }
    }
}

fn main() {
    let sam = Student {
        name: String::from("Sam"),
        locker_no: Some(32),
    };
    let dan = Student {
        name: String::from("Dan"),
        locker_no: None,
    };

    let students = vec![sam, dan];

    for s in students {
        s.print_locker()
    }
}
