use rust_learning::{TodoError, TodoList};

fn main() {
    let mut todo_list = TodoList::new();
    todo_list.add("Buy groceries".to_string()).unwrap();
    todo_list.add("Buy a new phone".to_string()).unwrap();
    let ids: Vec<u32> = todo_list.items.iter().map(|todo| todo.id).collect();
    for id in ids {
        match todo_list.complete(id) {
            Ok(_) => println!("Todo completed"),
            Err(TodoError::NotFound(id)) => println!("Todo #{} not found", id),
            Err(TodoError::EmptyTitle) => println!("Title cannot be empty"),
        }
    }
    todo_list.list();
}
