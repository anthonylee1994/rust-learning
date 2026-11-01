use std::{fmt, num};

struct Todo {
    id: u32,
    title: String,
    done: bool,
}

struct TodoList {
    items: Vec<Todo>,
}

#[derive(Debug)]
enum TodoError {
    NotFound(u32),
    EmptyTitle,
}

#[derive(Debug)]
enum AppError {
    Parse(std::num::ParseIntError),
    Todo(TodoError),
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoError::NotFound(id) => write!(f, "todo #{} not found", id),
            TodoError::EmptyTitle => write!(f, "title cannot be empty"),
        }
    }
}

impl From<num::ParseIntError> for AppError {
    fn from(value: num::ParseIntError) -> Self {
        AppError::Parse(value)
    }
}

impl From<TodoError> for AppError {
    fn from(value: TodoError) -> Self {
        AppError::Todo(value)
    }
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
    let mut todo_list = TodoList::new();
    todo_list.add("todo".to_owned()).unwrap();

    for input in ["1", "abc", "99"] {
        match complete_by_text(&mut todo_list, input) {
            Ok(()) => println!("todo #{} completed", input),
            Err(AppError::Parse(e)) => println!("'{}' is not a number: {}", input, e),
            Err(AppError::Todo(e)) => println!("todo error: {}", e),
        }
    }

    todo_list.list();
}

fn complete_by_text(list: &mut TodoList, input: &str) -> Result<(), AppError> {
    let id = input.parse::<u32>()?;
    list.complete(id)?;

    Ok(())
}
