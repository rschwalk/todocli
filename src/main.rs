use std::env;

struct TodoItem {
    name: String,
    completed: char
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        return TodoItem {
            name: name,
            completed: ' '
        };
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = arguments[1].clone();
    let todo_item1 = TodoItem::new("Todo1".to_string());
    let todo_item2 = TodoItem::new("Todo2".to_string());
    let todo_list = vec![todo_item1, todo_item2];

    if command == "get" {
        for item in todo_list {
            println!("[{}] - {}", item.completed, item.name);
        }
    }
}
