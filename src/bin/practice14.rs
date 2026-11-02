mod todo {
    use std::fmt;

    #[derive(Debug, PartialEq)]
    pub enum TodoError {
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

    pub struct Todo {
        id: u32,
        title: String,
        done: bool,
    }

    pub struct TodoList {
        items: Vec<Todo>,
    }

    impl TodoList {
        pub fn new() -> Self {
            Self { items: Vec::new() }
        }

        pub fn add(&mut self, title: String) -> Result<u32, TodoError> {
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

        pub fn complete(&mut self, id: u32) -> Result<(), TodoError> {
            let todo = self
                .items
                .iter_mut()
                .find(|todo| todo.id == id)
                .ok_or(TodoError::NotFound(id))?;
            todo.done = true;
            Ok(())
        }

        pub fn list(&self) {
            for todo in &self.items {
                let mark = if todo.done { "x" } else { " " };
                println!("[{}] #{} {}", mark, todo.id, todo.title);
            }
        }
    }
}

fn main() {
    use todo::{TodoError, TodoList};

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

#[cfg(test)]
mod tests {
    use super::todo::*;

    #[test]
    fn add_rejects_empty_title() {
        let mut list = TodoList::new();
        let result = list.add("".to_string());
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), TodoError::EmptyTitle);
    }
}
