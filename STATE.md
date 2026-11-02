# Rust Learning Progress

Background: TypeScript + Ruby on Rails developer learning Rust.
Last updated: 2026-11-02 (Round 4 done)

## Round 1: Basics (completed)

All files live in `src/bin/`. Run with `cargo run --bin practiceN`.

| #   | Topic                              | File           | Status | Notes                                                                                                                 |
| --- | ---------------------------------- | -------------- | ------ | --------------------------------------------------------------------------------------------------------------------- |
| 1   | Basic function (`f_to_c`)          | `practice1.rs` | Done   | Implicit return (no semicolon on last line)                                                                           |
| 2   | `Option` + `match` (`first_even`)  | `practice2.rs` | Done   | `.iter().find(...).copied()`                                                                                          |
| 3   | Ownership (borrow vs clone)        | `practice3.rs` | Done   | Uses `&name` and `&str` param; `.clone()` version is commented out                                                    |
| 4   | `enum` + `match` (`Shape`, `area`) | `practice4.rs` | Done   | Circle area was wrong at first (used circumference), fixed to `r * r * PI` with `f64` PI (prints `314.1592653589793`) |
| 5   | `Result` + `?` (`parse_age`)       | `practice5.rs` | Done   | Tested with `"25"` and `"abc"`                                                                                        |

## Round 2: Structs, traits, iterators (completed)

Questions for both rounds are in `QUESTIONS.md`.

| #   | Topic                                | File            | Status | Notes                                                                                       |
| --- | ------------------------------------ | --------------- | ------ | ------------------------------------------------------------------------------------------- |
| 6   | `struct` + `impl` (`Rectangle`)      | `practice6.rs`  | Done   | `new` (associated fn), `&self` vs `&mut self`                                               |
| 7   | `trait` (`Describe`)                 | `practice7.rs`  | Done   | `&impl Describe`. Hit `vec![rect, circle]` error, then solved with `Vec<Box<dyn Describe>>` |
| 8   | Iterators (`filter`/`map`/`collect`) | `practice8.rs`  | Done   | `sum`, `any`, `all`. Explained `iter` / `iter_mut` / `into_iter`                            |
| 9   | `struct` + iterators (`Student`)     | `practice9.rs`  | Done   | `max_by_key` returns `Option`, average with `as f64`. Used an array instead of `Vec`        |
| 10  | Mini todo list (`TodoList`)          | `practice10.rs` | Done   | `iter_mut().find(...).ok_or(...)?` returning `Result`. `add` generates the id itself        |

## Concepts covered

- Functions and implicit return
- `Option<T>` and `match` on `Some` / `None`
- Ownership, `&` borrowing, `.clone()`
- `enum` with tuple and struct variants, exhaustive `match`
- `Result<T, E>` and the `?` operator
- `struct`, `impl`, associated functions, `&self` vs `&mut self`
- `trait` and `&impl Trait` parameters
- Iterator chains: `filter`, `map`, `collect`, `sum`, `any`, `all`, `max_by_key`
- `iter()` vs `iter_mut()` vs `into_iter()` and how they relate to ownership
- Supertraits, `Box<dyn Trait>`
- Custom error enums, `Display`, `From` and `?` conversion
- Inline `mod` and `pub`, `#[cfg(test)]`, `#[test]`, `assert!` / `matches!`
- `std::fs` (`read_to_string`, `File::create`), `io::ErrorKind`, `io::Write` must be in scope
- `std::env::args`, slice patterns (`[_, cmd, rest @ ..]`) with match guards
- `match` arms must all return the same type (use `;` in a block to get `()`)

## Known loose ends

- `save` uses `File::create`, so a crash halfway leaves a truncated `todos.txt` (fix idea: write a temp file, then `rename`).
- `add` makes the id from `len() + 1`, which would clash if delete is ever added.
- Old practice files (10 to 15) still carry their own copy of the todo code; only practice20 uses `src/lib.rs`.
- Think-about answers for practice14 (`pub` privacy error, `use super::...`) are still not answered.

## Round 3: Traits, errors, modules, tests, CLI (completed)

Questions are in `QUESTIONS.md`.

| #   | Topic                        | File            | Status | Notes                                                                                                         |
| --- | ---------------------------- | --------------- | ------ | ------------------------------------------------------------------------------------------------------------- |
| 11  | Supertraits, `impl` vs `dyn` | `practice11.rs` | Done   | `trait Shape: Describe`, `Vec<Box<dyn Shape>>`, `map` + `sum`, good `think about` answers                     |
| 12  | Custom error types           | `practice12.rs` | Done   | `TodoError` + `Display`, `ok_or(...)?`, `main` triggers both error paths                                      |
| 13  | `From` and `?` conversion    | `practice13.rs` | Done   | `?` converts via `From`; `main` matches `AppError::Parse` and `AppError::Todo` separately                     |
| 14  | Modules and `cargo test`     | `practice14.rs` | Done   | inline `mod todo` with `pub`, 4 tests pass (`cargo test --bin practice14`)                                    |
| 15  | Todo CLI + file persistence  | `practice15.rs` | Done   | `args.as_slice()` match guards, `read_to_string` + `NotFound`, `Io` / `ParseBool` errors, `run()` + `exit(1)` |

Watch points from earlier rounds: finish every bullet of an exercise before review (skipped `list`, `any`/`all`, `collect` before), and give `ok_or` the error itself, not `Err(...)`.

## Round 4: Easy ones (completed)

Questions are in `QUESTIONS.md`.

| #   | Topic                        | File            | Status | Notes                                                                                                                         |
| --- | ---------------------------- | --------------- | ------ | ----------------------------------------------------------------------------------------------------------------------------- |
| 16  | `derive`, `if let`, `Option` | `practice16.rs` | Done   | `Option<&User>` with two ref params needs a lifetime, so returned `Option<User>` via `.cloned()`. Compare with `==` before move |
| 17  | `HashMap` (word count)       | `practice17.rs` | Done   | `*map.entry(w).or_insert(0) += 1`, `.iter().collect()` into `Vec<(&String, &usize)>`, `sort_by`. Keep the map in a variable   |
| 18  | Generics                     | `practice18.rs` | Done   | `largest<T: PartialOrd + Copy>`, `Pair<T>`. Tested `i32`, `f64`, `&str` (not `char`)                                          |
| 19  | Trait default method         | `practice19.rs` | Done   | `intro` default uses `name()` / `speak()`, `Dog` overrides. Default text reads oddly for `Cat` ("I'm a meow")                 |
| 20  | Shared `src/lib.rs`          | `practice20.rs` | Done   | `use rust_learning::{...}`, 4 tests moved to `lib.rs` (`cargo test`). Fixed E0499 by collecting ids before the loop           |

New concepts: `#[derive(Debug, Clone, PartialEq)]`, `if let` vs `match`, `HashMap` and `entry`, generics with trait bounds, default trait methods, `lib.rs` shared by `src/bin/*`, E0499 (cannot borrow mutably twice, collect first then mutate).

Next: Round 5 (not planned yet). Candidates: `serde` / `clap`, `impl Trait` and closures, then lifetimes.

Reminders: avoid lifetimes and `async` for now, and `.clone()` freely when stuck.
