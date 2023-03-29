mod terminal;
mod todo;

use terminal::*;
use todo::*;

fn main() {
    print_app_name();

    print_input_title();
    let title = read_title();
    clear_output();

    print_input_priority();
    let priority = read_priority();
    clear_output();

    let task = Task::from(title, Priority::from(priority));

    let mut tasks: Vec<Task> = Vec::new();

    tasks.push(task);

    for task in &tasks {
        println!("{}", task.show())
    }
}
