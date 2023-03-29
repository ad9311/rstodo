mod todo;

use todo::*;

fn main() {
    println!("RSTodo");

    let task1 = Task::new(String::from("Do the dishes"), Priority::High);
    let task2 = Task::new(String::from("Buy cat food"), Priority::Normal);
    let task3 = Task::new(String::from("Finish paper"), Priority::Low);
    let task4 = Task::new(String::from("Meditate"), Priority::None);

    let mut tasks: Vec<Task> = Vec::new();

    tasks.push(task1);
    tasks.push(task2);
    tasks.push(task3);
    tasks.push(task4);

    let s = "Ángel";
    for b in s.as_bytes() {
        println!("{}", b);
    }

    for task in &tasks {
        println!("{}", task.show());
    }
}
