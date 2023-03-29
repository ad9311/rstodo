mod todo;

use std::io::stdin;
use todo::*;

fn main() {
    println!("RSTodo");
    println!("\nNew task");
    println!("Enter task title:");

    let mut title = String::new();
    stdin()
        .read_line(&mut title)
        .expect("Error reading user input");
    title = title.trim().to_string();

    println!("\nEnter task priority");
    println!("(h=high, n=normal, l=low, d=none[default]):");
    let mut priority = String::new();
    stdin()
        .read_line(&mut priority)
        .expect("Error reading user input");
    priority = priority.trim().to_string();

    println!("{} - {}", title, priority);

    let task = Task::new(title, Priority::from(priority));

    let mut tasks: Vec<Task> = Vec::new();

    tasks.push(task);

    for task in &tasks {
        println!("{}", task.show());
    }
}
