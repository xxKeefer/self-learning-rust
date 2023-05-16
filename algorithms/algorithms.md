# Algorithms in Rust

A Collection of Notes I took will working through [The Last Algorithms Course You'll Need - The Primeagen](https://frontendmasters.com/courses/algorithms/introduction/) on Frontend Masters.

I chose to work through the course and reproduce the exercises in Rust to learn the language better. Some of the notes are not in depth nor complete, if I previously understood a topic I will not have taken notes on it.

## Big O Notation

Big O notation is a way to describe the performance of an algorithm. It is a way to describe how the runtime of an algorithm grows as the input grows. It's not a precise measurement, but a way to describe the general trend of the algorithm.

### The Trick to Big O

 > Count the nested loops.

 consider the following code:

 ```rust
 fn sum_char_codes(string: &str) -> u32 {
     let mut sum = 0;
     for char in string.chars() {
         sum += char as u32;
     }
     sum
 }
 ```

 one loop means a worst case Big O time complexity of `O(n)`. If you add more characters to the string, the runtime will grow linearly.

 ```rust
    fn sum_char_codes(string: &str) -> u32 {
        let mut sum = 0;
        for char in string.chars() {
            sum += char as u32;
        }
        for char in string.chars() {
            sum += char as u32;
        }
        sum
    }
```

two loops means a worst case Big O time complexity of `O(2n)`. The `2` is dropped because it is a constant, and constants are dropped in Big O notation as they effect the running time negligibly.

 ```rust
    fn sum_char_codes(string: &str) -> u32 {
    let mut sum = 0;
    for _ in string.chars() {
        for char in string.chars() {
            sum += char as u32;
        }
    }
    sum
}
```

two nested loops means a worst case Big O time complexity of `O(n^2)`. The `2` is kept because it is not a constant, it is a variable that will drastically increase the time complexity. here are the growth rates of common Big O notations:

![bigO graph](./assets/bigO.png)

## Deciding on an Algorithm

### is your data sorted?

If your data is sorted, you can use a binary search to find the data you are looking for. A binary search is `O(log n)`.
