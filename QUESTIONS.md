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

---

# Round 3 Questions

Theme: finish traits properly, then move from toy code to a real, tested, multi-file-style CLI.
Same rules as before: one file per exercise in `src/bin/practiceN.rs`, each with its own `main` that prints visible output.

Checklist before asking for review (earlier rounds lost time on these):

- Re-read the exercise and tick every bullet, including the "Think about" questions.
- Run it once with the happy path and once with the failure path.
- Read all compiler warnings, not only errors.

## practice11: Supertraits and `impl` vs `dyn`

In round 2 you made `Describe` work with `Box<dyn Describe>`. Now make shapes properly.

- Define `trait Shape: Describe { fn area(&self) -> f64; }`. The `: Describe` part is a supertrait, like TS `interface Shape extends Describe`.
- Implement `Describe` and `Shape` for `Rectangle` and `Circle`.
- Write `fn total_area(shapes: &[Box<dyn Shape>]) -> f64` using an iterator (`map` + `sum`).
- In `main`, build a `Vec<Box<dyn Shape>>` with at least 3 shapes, print each `describe()` (which comes from the supertrait!) and the total area.
- Write a second function `fn print_area(shape: &impl Shape)` and call it with one concrete shape.

**Think about**: why can you call `describe()` on a `Box<dyn Shape>`? When would you pick `impl Shape` over `dyn Shape`? Write your answer as a comment at the bottom of the file.

## practice12: Custom error types

`Result<(), String>` works, but callers cannot tell errors apart. Use a real error type, like a Ruby exception class hierarchy.

- Define `enum TodoError { NotFound(u32), EmptyTitle }`.
- Implement `std::fmt::Display` for it (`fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result`, use `write!`).
- Copy `TodoList` from practice10 and change it:
  - `add(&mut self, title: String) -> Result<u32, TodoError>`: reject an empty or whitespace-only title (use `trim()`), otherwise return the new id.
  - `complete(&mut self, id: u32) -> Result<(), TodoError>`: use `ok_or(TodoError::NotFound(id))?`.
- In `main`, trigger both errors. Use `match` on the error to print a different message per variant, then print the other one with `{}` through `Display`.

**Think about**: what does the caller gain compared to a `String` error? (Hint: `match` is exhaustive. Add a third variant and see what the compiler tells you.)

## practice13: The `From` trait and `?` conversion

`?` converts errors automatically if a `From` impl exists. You saw this when `&str` became `String`. Now write your own.

- Define `enum AppError { Parse(std::num::ParseIntError), Todo(TodoError) }` (copy `TodoError` and its `Display` from practice12).
- Implement `From<std::num::ParseIntError> for AppError` and `From<TodoError> for AppError`.
- Write `fn complete_by_text(list: &mut TodoList, input: &str) -> Result<(), AppError>`:
  - parse `input` into a `u32` with `?`
  - call `list.complete(id)` with `?`
  - no `map_err` allowed, the `?` must convert for you
- In `main`, call it with a valid id (`"1"`), a non-number (`"abc"`) and a missing id (`"99"`), printing the result each time.

**Think about**: which line does the `?` conversion happen on, and which trait makes it work?

## practice14: Modules and `cargo test`

- Put your practice12 types inside an inline module: `mod todo { pub struct Todo ... pub struct TodoList ... pub enum TodoError ... }`. Mark only what `main` needs as `pub` (including fields or methods it touches).
- In `main`, bring things in with `use todo::{TodoList, TodoError};`.
- Add a test module at the bottom:

```rust
#[cfg(test)]
mod tests {
    use super::todo::*;

    #[test]
    fn add_rejects_empty_title() { /* ... */ }
}
```

- Write at least 4 tests: empty title rejected, ids increase, `complete` marks the item done, `complete` on a missing id returns `NotFound`.
- Run with `cargo test --bin practice14`.

**Think about**: what happens if you remove `pub` from one method? Read the privacy error. What does `use super::...` mean inside `mod tests`?

## practice15: Todo CLI with file persistence (mini project)

Turn the todo list into a real command-line tool. Keep it in one file, `src/bin/practice15.rs`, using what you built in practice12 to practice14.

Commands (read from `std::env::args()`):

```text
cargo run --bin practice15 -- add "buy milk"
cargo run --bin practice15 -- list
cargo run --bin practice15 -- done 1
```

Requirements:

- Storage is a plain text file `todos.txt` in the current directory, one todo per line, like `1|false|buy milk`. No `serde` yet, parse by hand with `split('|')`.
- `load() -> Result<TodoList, AppError>`: if the file does not exist, return an empty list (not an error). Parse each line with `?`.
- `save(&TodoList) -> Result<(), AppError>`.
- Add `Io(std::io::Error)` to `AppError` with a `From` impl so `std::fs::read_to_string` and `std::fs::write` work with `?`.
- `main` calls `run() -> Result<(), AppError>`, and prints the error with `{}` and exits with code 1 if it fails (`std::process::exit(1)`).
- Unknown or missing command prints a short usage message.
- Add `todos.txt` to `.gitignore`.

Stuck points to expect: `args().collect::<Vec<String>>()`, matching on a slice (`match args.as_slice() { [_, cmd, rest @ ..] => ... }`), and `.lines()` on a `String`. Ask for hints on any of them.

**Think about**: what happens to your data if the process crashes halfway through `save`? You do not need to fix it, just note it.

## Round 4: Easy ones (derive, HashMap, generics, `if let`, `lib.rs`)

Each one is short. One concept per file.

## practice16: `derive` and `Option` helpers

Make a `struct User { name: String, age: u32 }` with `#[derive(Debug, Clone, PartialEq)]`.

- In `main`, create two users with the same data and check `==`.
- Print one with `{:?}` and `{:#?}`.
- Write `find_user(users: &[User], name: &str) -> Option<User>`. Return a clone (`.cloned()` on the `find` result), so no lifetime is needed.
- Use `if let Some(u) = ...` for the found case, and `.map(|u| u.age).unwrap_or(0)` for the missing case.

**Think about**: when do you pick `if let` over a full `match`?

## practice17: `HashMap` (word count)

Write `count_words(text: &str) -> HashMap<String, usize>`.

- Split with `split_whitespace()`, lowercase each word.
- Use `*map.entry(word).or_insert(0) += 1;`.
- In `main`, count `"the cat and the dog and the bird"` and print the result sorted by count (descending). Collect into a `Vec<(&String, &usize)>` first, then `sort_by`.

**Think about**: what is the Rails or TS equivalent of `entry().or_insert()`?

## practice18: Generics

- Write `fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T>`. Test with `i32`, `f64` and `char`.
- Write `struct Pair<T> { a: T, b: T }` with `impl<T: PartialOrd + std::fmt::Display> Pair<T>` and a method `print_larger(&self)`.

**Think about**: what does the compiler say if you remove `Copy` from the bound? Read the error fully.

## practice19: Trait with a default method

- `trait Animal { fn name(&self) -> String; fn speak(&self) -> String; fn intro(&self) -> String { ... } }`, where `intro` has a default body that uses `name()` and `speak()`.
- Implement it for `Dog` and `Cat`. Override `intro` for only one of them.
- Put both into a `Vec<Box<dyn Animal>>` and print every `intro()`.

**Think about**: how is this different from an abstract class in Ruby or TS?

## practice20: Shared `src/lib.rs`

Fix the loose end that the todo code is copied around.

- Create `src/lib.rs` and move `Todo`, `TodoList`, `TodoError` into it (all `pub`).
- In a new `src/bin/practice20.rs`, use it with `use rust_learning::{TodoList, TodoError};` (check the crate name in `Cargo.toml`, `-` becomes `_`).
- Move your 4 tests from practice14 into `lib.rs`, run with `cargo test`.

**Think about**: why can binaries in `src/bin/` import from `lib.rs` but not from each other?

## Reminders

- No lifetimes or `async` yet. If the compiler asks for a lifetime, switch the parameter to `String` or `.clone()`.
- Do one exercise at a time and ask for review after each. Hints are fine, just ask.
- `cargo fmt` before asking for review. Do not run `cargo clippy` unless you want it.
