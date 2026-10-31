# Rust Learning Progress

Background: TypeScript + Ruby on Rails developer learning Rust.
Last updated: 2026-10-31 (Round 2 done)

## Round 1: Basics (completed)

All files live in `src/bin/`. Run with `cargo run --bin practiceN`.

| #   | Topic                              | File           | Status | Notes                                                                      |
| --- | ---------------------------------- | -------------- | ------ | -------------------------------------------------------------------------- |
| 1   | Basic function (`f_to_c`)          | `practice1.rs` | Done   | Implicit return (no semicolon on last line)                                |
| 2   | `Option` + `match` (`first_even`)  | `practice2.rs` | Done   | `.iter().find(...).copied()`                                               |
| 3   | Ownership (borrow vs clone)        | `practice3.rs` | Done   | Uses `&name` and `&str` param; `.clone()` version is commented out         |
| 4   | `enum` + `match` (`Shape`, `area`) | `practice4.rs` | Done   | Circle area was wrong at first (used circumference), fixed to `r * r * PI` with `f64` PI (prints `314.1592653589793`) |
| 5   | `Result` + `?` (`parse_age`)       | `practice5.rs` | Done   | Tested with `"25"` and `"abc"`                                             |

## Round 2: Structs, traits, iterators (completed)

Questions for both rounds are in `QUESTIONS.md`.

| #   | Topic                                  | File            | Status | Notes                                                                                   |
| --- | -------------------------------------- | --------------- | ------ | --------------------------------------------------------------------------------------- |
| 6   | `struct` + `impl` (`Rectangle`)        | `practice6.rs`  | Done   | `new` (associated fn), `&self` vs `&mut self`                                           |
| 7   | `trait` (`Describe`)                   | `practice7.rs`  | Done   | `&impl Describe`. Hit `vec![rect, circle]` error, then solved with `Vec<Box<dyn Describe>>` |
| 8   | Iterators (`filter`/`map`/`collect`)   | `practice8.rs`  | Done   | `sum`, `any`, `all`. Explained `iter` / `iter_mut` / `into_iter`                        |
| 9   | `struct` + iterators (`Student`)       | `practice9.rs`  | Done   | `max_by_key` returns `Option`, average with `as f64`. Used an array instead of `Vec`    |
| 10  | Mini todo list (`TodoList`)            | `practice10.rs` | Done   | `iter_mut().find(...).ok_or(...)?` returning `Result`. `add` generates the id itself    |

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

## Known loose ends

- Only `Todo` in memory, no persistence and no real CLI input yet.

## Round 3: Traits, errors, modules, tests, CLI (in progress)

Questions are in `QUESTIONS.md`. Planned exercises:

- `practice11`: supertraits (`trait Shape: Describe`), `Vec<Box<dyn Shape>>`, `impl` vs `dyn`
- `practice12`: custom error enum, `Display`, `ok_or(TodoError::NotFound(id))?`
- `practice13`: `From` impls so `?` converts errors (`ParseIntError` and `TodoError` into `AppError`)
- `practice14`: inline `mod` with `pub`, `use`, and `cargo test --bin practice14`
- `practice15`: todo CLI with `std::env::args`, file persistence in `todos.txt`, `Io` error variant

Watch points from earlier rounds: finish every bullet of an exercise before review (skipped `list`, `any`/`all`, `collect` before), and give `ok_or` the error itself, not `Err(...)`.

Reminders: avoid lifetimes and `async` for now, and `.clone()` freely when stuck.
