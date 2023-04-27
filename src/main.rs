mod terminal;
mod todo;

use terminal::*;
use todo::*;

fn main() {
    print_app_name();

    let mut tasks: Vec<Task> = Vec::new();

    loop {
        print_input_title();
        let title = read_title();
        if title.is_empty() {
            alert_empty_title();
            continue;
        }
        clear_output();
        print_app_name();

        print_input_priority();
        let priority = read_priority();
        clear_output();
        print_app_name();

        let task = Task::from(title, Priority::from(priority));

        tasks.push(task);

        for task in &tasks {
            println!("{}", task.show())
        }
    }
}
