# Round 1 Questions

## Exercise 1: Basic function

Write `fn f_to_c(f: f64) -> f64` that converts Fahrenheit to Celsius. Print `f_to_c(98.6)` in `main`.

Hint: the last line without a semicolon is the return value, like Ruby's implicit return.

## Exercise 2: Option

Write `fn first_even(nums: &[i32]) -> Option<i32>` that finds the first even number. In `main`, handle `Some` and `None` with `match`.

Hint: you can use `for n in nums`, or `nums.iter().find(...)`.

## Exercise 3: Ownership (fix the compile error)

```rust
fn main() {
    let name = String::from("Ferris");
    greet(name);
    println!("{}", name);
}

fn greet(n: String) {
    println!("Hi {}", n);
}
```

Fix it in two ways: (a) borrow with `&`, (b) use `.clone()`. Then think about which one is better and why.

## Exercise 4: enum + match

Define:

```rust
enum Shape {
    Circle(f64),
    Rect { w: f64, h: f64 },
}
```

Write `fn area(s: &Shape) -> f64`. Then try adding a `Triangle` variant to the enum and see how the compiler reminds you.

## Exercise 5: Result and `?`

Write `fn parse_age(s: &str) -> Result<u32, std::num::ParseIntError>` using `s.parse::<u32>()` and `?`. Try `"25"` and `"abc"` in `main`.

---

# Round 2 Questions

Files live in `src/bin/practiceN.rs`. Run with `cargo run --bin practiceN`.
Each file has its own `main` and prints visible output.

## practice6: `struct` + `impl`

A struct is like a TS `interface` and a class's methods combined, except data and behavior are written separately: `struct` holds the fields, `impl` holds the methods.

- Define `struct Rectangle { width: f64, height: f64 }`
- In `impl`, write:
  - `fn new(width: f64, height: f64) -> Rectangle`: an associated function, like Rails' `self.build`, with no `self` parameter
  - `fn area(&self) -> f64`
  - `fn scale(&mut self, factor: f64)`: mutates itself
- In `main`, create one, print its area, scale it, then print the area again

**Think about**: why does `area` take `&self` while `scale` takes `&mut self`? Why does the `let` need `mut`?

## practice7: `trait`

A trait is like a TS `interface`, but you can also add implementations to your own types.

- Define `trait Describe { fn describe(&self) -> String; }`
- Implement `Describe` for practice6's `Rectangle` and for a new struct `Circle { radius: f64 }`
- Write `fn print_it(item: &impl Describe)` and call it with both types

**Think about**: if you want to put a Rectangle and a Circle in the same `Vec`, is `impl Describe` enough? (You will hit a wall. Read what the error says. The fix is for round 3, so this time just note the error down.)

## practice8: Iterator basics

An iterator is like a JS `.map().filter()` chain, but lazy: nothing runs until you `collect()`.

Use `let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];`:

1. Pick the even numbers, square them, and `collect` into a `Vec<i32>`
2. Use `.sum::<i32>()` to get the total
3. Try `.iter().any(...)` and `.all(...)` once each

**Think about**: what is the difference between `.iter()`, `.into_iter()` and `.iter_mut()`? How does each relate to ownership?

## practice9: Struct + iterator together

- Define `struct Student { name: String, score: u32 }`
- Build a `Vec<Student>` (at least 4 students)
- Use iterators to:
  1. Pick students with score >= 60 and `collect` their names into a `Vec<String>`
  2. Compute the average score (remember the `f64` conversion)
  3. Find the student with the highest score (hint: `max_by_key`, whose result is an `Option` that you already know how to handle)

## practice10: Mini CLI project (warm-up)

Write an in-memory todo list (no saving to disk yet):

- `struct Todo { id: u32, title: String, done: bool }`
- `struct TodoList { items: Vec<Todo> }` with `add`, `complete(id) -> Result<(), String>` and `list`
- In `main`, run a series of operations: add three items, complete one, complete an id that does not exist (the `Err` must be handled), and finally print everything

**Key point**: `complete` should use `iter_mut().find(...)` with `ok_or(...)?` or a `match`, tying round 1's `Option`/`Result` together with this round's material.

## Reminders

- When stuck, just `.clone()`. Do not waste time making it pretty.
- Read the whole compiler error first. If it still does not make sense, paste it and we will go through it line by line.
- Save each exercise as you finish it and ask for a review. No need to wait for all five.
