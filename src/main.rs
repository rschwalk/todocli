use std::env;

struct TodoItem {
    name: String,
    completed: char
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        TodoItem {
            name: name,
            completed: ' '
        }
    }
}

struct TodoList {
    list: Vec<TodoItem>
}

impl TodoList {
    fn new() -> TodoList {
        TodoList{ list: Vec::new()}
    }
    
    fn add_to_list(&mut self, name: String) {
        let todo_item = TodoItem::new(name);
        self.list.push(todo_item);
    }

    fn print(&self) {
        for item in &self.list {
            println!("[{}] - {}", item.completed, item.name);
        }
    }
}

enum Commands {
    Get, 
    Add(String)
}

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let command = match arguments[1].as_str() {
        "get" => Commands::Get,
        "add" => Commands::Add(arguments[2].clone()),
        _ => panic!("you must provide an accepdet command")
    };
    
    let mut todo_list = TodoList::new();

    todo_list.add_to_list("Todo1".to_string());
    todo_list.add_to_list("Todo2".to_string());

    match command {
        Commands::Get => todo_list.print(),
        Commands::Add(task) => {
            todo_list.add_to_list(task);
            todo_list.print();    
        }
    }
}
