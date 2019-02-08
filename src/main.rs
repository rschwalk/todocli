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
        for (index, item) in self.list.iter().enumerate() {
            println!("{} [{}] - {}", index, item.completed, item.name);
        }
    }

    fn mark_done(&mut self, index: usize) {
        self.list[index].completed = 'x';
    }
}

enum Commands {
    Get, 
    Add(String),
    Done(usize)
}

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let command = match arguments[1].as_str() {
        "get" => Commands::Get,
        "add" => Commands::Add(arguments[2].clone()),
        "done" => Commands::Done(arguments[2].parse().expect("error converting to integer")),
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
        },
        Commands::Done(index) => {
            todo_list.mark_done(index);
            todo_list.print();
        }
    }
}
