use std::fmt;

#[derive(Debug)]
enum TodoError {
    NotFound(u32),
    EmptyTitle,
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoError::NotFound(id) => write!(f, "todo #{} not found", id),
            TodoError::EmptyTitle => write!(f, "title cannot be empty"),
        }
    }
}

struct Todo {
    id: u32,
    title: String,
    done: bool,
}

struct TodoList {
    items: Vec<Todo>,
}

impl TodoList {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn add(&mut self, title: String) -> Result<u32, TodoError> {
        if title.trim().is_empty() {
            return Err(TodoError::EmptyTitle);
        }

        let id = self.items.len() as u32 + 1;
        self.items.push(Todo {
            id,
            title,
            done: false,
        });
        Ok(id)
    }

    fn complete(&mut self, id: u32) -> Result<(), TodoError> {
        let todo = self
            .items
            .iter_mut()
            .find(|todo| todo.id == id)
            .ok_or(TodoError::NotFound(id))?;
        todo.done = true;
        Ok(())
    }

    fn list(&self) {
        for todo in &self.items {
            let mark = if todo.done { "x" } else { " " };
            println!("[{}] #{} {}", mark, todo.id, todo.title);
        }
    }
}

fn main() {
    let mut todos = TodoList::new();

    todos.add(String::from("buy milk")).unwrap();
    todos.add(String::from("learn Rust")).unwrap();
    todos.add(String::from("write tests")).unwrap();

    // Happy path: complete an item that exists.
    match todos.complete(1) {
        Ok(()) => println!("completed #1"),
        Err(error) => println!("failed: {}", error),
    }

    // Error path 1: missing id, matched per variant so each one can print differently.
    match todos.complete(99) {
        Ok(()) => println!("completed #99"),
        Err(TodoError::NotFound(id)) => println!("no such todo: #{}", id),
        Err(TodoError::EmptyTitle) => println!("title cannot be empty"),
    }

    // Error path 2: empty title, printed through Display with `{}`.
    if let Err(error) = todos.add(String::from("   ")) {
        println!("add failed: {}", error);
    }

    todos.list();
}

// Think about: what does the caller gain compared to a `String` error?
// The caller can `match` on the variants, and `match` is exhaustive: if a new
// variant is added, the compiler points at every caller that must handle it.
// With `String` you would only find out at run time by inspecting the text.
