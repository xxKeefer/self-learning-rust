// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum TeaKind {
    Lemon,
    Herbal,
    Black,
    Green,
}

enum DrinkSize {
    Sm,
    Md,
    Lg,
}

struct TeaDrink {
    kind: TeaKind,
    size: DrinkSize,
}

fn print_drink(drink: TeaDrink) {
    let kind = match drink.kind {
        TeaKind::Black => "Black Tea",
        TeaKind::Lemon => "Lemon Tea",
        TeaKind::Herbal => "Herbal Tea",
        TeaKind::Green => "Green Tea",
    };

    let size = match drink.size {
        DrinkSize::Lg => "large",
        DrinkSize::Md => "medium",
        DrinkSize::Sm => "small",
    };

    println!("You have selected a {} {}!", size, kind)
}

fn main() {
    let black = TeaDrink {
        size: DrinkSize::Lg,
        kind: TeaKind::Black,
    };
    print_drink(black);

    let herbal = TeaDrink {
        size: DrinkSize::Md,
        kind: TeaKind::Herbal,
    };
    print_drink(herbal);

    let green = TeaDrink {
        size: DrinkSize::Sm,
        kind: TeaKind::Green,
    };
    print_drink(green);

    let lemon = TeaDrink {
        size: DrinkSize::Lg,
        kind: TeaKind::Lemon,
    };
    print_drink(lemon);
}
