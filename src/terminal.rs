use std::io::{stdin, stdout, Write};
use termion::{clear, cursor};

pub fn print_app_name() {
    println!("RSTodo")
}

pub fn clear_output() {
    let result = write!(stdout(), "{}{}", clear::All, cursor::Goto(1, 1));
    if result.is_err() {
        println!("Could not clear the terminal")
    }
}

pub fn print_input_title() {
    println!("Enter task title:")
}

pub fn print_input_priority() {
    println!("Enter task priority");
    println!("(h=high, n=normal, l=low, none[default]):")
}

pub fn read_title() -> String {
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    title.trim().to_owned()
}

pub fn read_priority() -> String {
    let mut priority = String::new();
    stdin().read_line(&mut priority).unwrap();
    priority.trim().to_owned()
}

pub fn alert_no_empty_title() {
    println!("The task's title must not be empty")
}
