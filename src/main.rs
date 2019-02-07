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

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = arguments[1].clone();
    let mut todo_list = TodoList::new();

    todo_list.add_to_list("Todo1".to_string());
    todo_list.add_to_list("Todo2".to_string());

    if command == "get" {
       todo_list.print();
    }
}
