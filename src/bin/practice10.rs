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
        Self { items: vec![] }
    }

    fn add(&mut self, title: String) {
        let id = self.items.len() as u32 + 1;
        self.items.push(Todo {
            id,
            title,
            done: false,
        });
    }

    fn complete(&mut self, id: u32) -> Result<(), String> {
        let item = self.items.iter_mut().find(|x| x.id == id);
        match item {
            Some(x) => {
                x.done = true;
                Ok(())
            }
            None => Err("Cannot find the item".to_string()),
        }
    }

    fn list(&self) {
        for item in self.items.iter() {
            println!(
                "ID: {}, Title: {}, Done: {}",
                item.id, item.title, item.done
            );
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    todo_list.add("A".to_owned());
    todo_list.add("B".to_owned());
    todo_list.add("C".to_owned());

    let first_complete = todo_list.complete(1);
    let second_complete = todo_list.complete(4);

    match first_complete {
        Ok(_) => println!("complete"),
        Err(e) => println!("{}", e),
    }

    match second_complete {
        Ok(_) => println!("complete"),
        Err(e) => println!("{}", e),
    }

    todo_list.list();
}
