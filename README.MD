# Rust Learnings

A Collection of Rust Notes and Projects I have made during my learning process, this not an indepth guide or even complete in some cases. It is a collection of notes that I made to help me learn Rust fro mmy perspect as a Typescript dev. If I already understood a concept, I did not take notes on it.

## Learning Resources

- [Rust 101 Crash Course: Learn Rust (6 HOURS!) + 19 Practice Exercises | Zero To Mastery](https://www.youtube.com/watch?v=lzKeecy4OmQ)

## Data Types

- **Boolean, Integer, Float and Double** : these i have seen before
- **Character** : `char` is a single unicode character, specified with single quotes
- **String** : `String` is a collection of characters, specified with double quotes. This what i normally think of as strings, the difference here is that there is a specific difference between a `char` and a `string`

## Variables

Variables assign data (a value and type) to a temporary memory location. These are immutable  by default, but can be made mutable with the `mut` keyword.

### Some examples

**Note**: Comments are the same as in javascript, single line comments are `//` and multiline comments are `/* */`

**Note**: The `let` keyword is used to declare a variable, and the semi colon `;` is used to end a statement and continue to the next line.

```rust
let two = 2; // this is an integer
let hello = "Hello"; // this is a string
let j = 'j'; // this is a char
let my_half = 0.5; // this is a float
let mut username = "John"; // this is a mutable string
let is_cool = true; // this is a boolean
let your_half = my_half // this sets your_half to the value of my_half
```

**Note**: Because `username` is mutable, we can change it's value, but I am not sure if your_half is a reference to my_half (like in javascript) or a copy of it.

## Functions

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

- `fn` is the keyword to declare a function
- `add` is the name of the function
- `x` and `y` are the parameters. They are typed, and the type is specified after the parameter name, with a colon `:`
- `-> i32` is the return type
- `x + y` is the return value. The last line of a function is the return value, and the `return` keyword is not needed
- the opening and closing curly braces `{}` are used to define the body of the function and defines a scope

## Macros

### The `println!` macro

This is kind of the equivalent of `console.log` in javascript. It is used to print to the console.
**Note**: The `!` at the end of the macro name is used to indicate that it is a macro, and not a function.

```rust
let life = 42;
println!("Hello World!");
println!("The answer to life, the universe and everything is {:?}", life);
```

the `{:?}` is used to print the value of a variable. It is called a `placeholder` and there are many different types of placeholders. the `{:?}` indicates a debug value. There is also `{}` which is the default, and `{:#?}` which is a pretty print.
You can add as many placeholders as you want, and they will be printed in order.

```rust
let life = 42;
let name = "John";
let hug = 'O';
let kiss = 'X';
println!("Hello World! My name is {} and the answer to life, the universe and everything is {:?}", name, life);
println!("{}{}{}{}{}{}", hug, kiss, hug, kiss, hug, kiss);
```

It is also possible to inline the placeholders.

```rust
let life = 42;
println!("The meaning of life id {life:?}"); // debug version
println!("The meaning of life id {life}"); // default version

```

## Control Flow

### If Statements

`if, else, else if` : pretty much the same as in javascript, a notable difference is that the condition does not need to be wrapped in parenthesis `()`

```rust
let x = 5;
if x == 5 {
    println!("x is 5");
} else if x == 6 {
    println!("x is 6");
} else {
    println!("x is not 5 or 6");
}
```

### Match

`match` : this is very simmilar to a `switch` statement in javascript. It is used to match a value to a pattern, and then execute the code block for that pattern. It is important it be exhaustive, meaning that it needs to cover all possible values. If it is not exhaustive, the compiler will throw an error.

```rust
let some_bool = true;
match some_bool {
    true => println!("it is true"),
    false => println!("it is false"),
    _ => println!("This matches all other cases, but is imposible for this boolean example."),
}
```

**Note**: Use `_` to match the default case, or all other option not covered by the exhaustive cases.
**Note**: `else if` is not checked by the compiler but `match` is. If a new variable is created that isn't handled by the match cases the comiler will yell at you. this is not the case with `if else`s

### Loops

`loop` : this is the equivalent of a `while true` loop in javascript. It will run forever, unless you break out of it.

`while` : this is the equivalent of a `while` loop in javascript. It will run until the condition is false.

```rust
let mut x = 0;
loop {
    println!("x is {}", x);
    x += 1;
    if x == 5 {
        break;
    }
}

let mut y = 0;
while y <= 5 {
    println!("y is {}", y);
    y += 1;
}
```
