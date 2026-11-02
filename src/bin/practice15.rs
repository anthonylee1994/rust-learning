use std::{
    fmt,
    fs::read_to_string,
    io::{ErrorKind, Write},
};

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
    ParseBool(std::str::ParseBoolError),
    InvalidLine(String),
    Todo(TodoError),
    Io(std::io::Error),
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoError::NotFound(id) => write!(f, "todo #{} not found", id),
            TodoError::EmptyTitle => write!(f, "title cannot be empty"),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Parse(e) => write!(f, "invalid number: {}", e),
            AppError::ParseBool(e) => write!(f, "invalid bool: {}", e),
            AppError::InvalidLine(line) => write!(f, "invalid line in todos.txt: {}", line),
            AppError::Todo(e) => write!(f, "{}", e),
            AppError::Io(e) => write!(f, "io error: {}", e),
        }
    }
}

impl From<std::str::ParseBoolError> for AppError {
    fn from(value: std::str::ParseBoolError) -> Self {
        AppError::ParseBool(value)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(value: std::num::ParseIntError) -> Self {
        AppError::Parse(value)
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::Io(value)
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

    fn load(&mut self) -> Result<(), AppError> {
        let contents = match read_to_string("todos.txt") {
            Ok(contents) => contents,
            Err(e) if e.kind() == ErrorKind::NotFound => return Ok(()),
            Err(e) => return Err(e.into()),
        };
        for line in contents.lines() {
            let parts = line.split('|').collect::<Vec<&str>>();
            if parts.len() != 3 {
                return Err(AppError::InvalidLine(line.to_string()));
            }
            let id = parts[0].parse::<u32>()?;
            let done = parts[1].parse::<bool>()?;
            let title = parts[2].to_string();
            self.items.push(Todo { id, title, done });
        }
        Ok(())
    }

    fn save(&self) -> Result<(), AppError> {
        let mut file = std::fs::File::create("todos.txt")?;
        for todo in &self.items {
            file.write_fmt(format_args!("{}|{}|{}\n", todo.id, todo.done, todo.title))?;
        }
        Ok(())
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
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  cargo run --bin practice15 -- add <title>");
    eprintln!("  cargo run --bin practice15 -- list");
    eprintln!("  cargo run --bin practice15 -- done <id>");
}

fn run() -> Result<(), AppError> {
    let args = std::env::args().collect::<Vec<String>>();
    let mut todos = TodoList::new();
    todos.load()?;

    match args.as_slice() {
        [_, cmd, title] if cmd == "add" => {
            let id = todos.add(title.clone())?;
            todos.save()?;
            println!("added #{}", id);
        }
        [_, cmd] if cmd == "list" => todos.list(),
        [_, cmd, id] if cmd == "done" => {
            todos.complete(id.parse::<u32>()?)?;
            todos.save()?;
            println!("completed #{}", id);
        }
        _ => print_usage(),
    }
    Ok(())
}
