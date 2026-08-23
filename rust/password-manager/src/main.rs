use std::fmt;
use std::io;

use clap::ArgAction::Count;
struct Task {
    id: u32,
    title: String,
    status: Status,
}

enum Status {
    Todo,
    InProgress,
    Done,
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Todo => write!(f, "todo"),
            Status::InProgress => write!(f, "in progress"),
            Status::Done => write!(f, "done"),
        }
    }
}
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}. {}:{}", self.id, self.title, self.status)
    }
}
fn input_str(prompt: &str) -> String {
    println!("{prompt}");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

fn input_int(prompt: &str) -> i32 {
    println!("{prompt}");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse().expect("Input was not a valid integer")
}

fn add_task(total_task: &mut Vec<Task>, count: u32) {
    let title = input_str("What is the name of the task you're adding?: ");
    println!("you've added {title}");
    total_task.push(Task {
        id: { count },
        title: title,
        status: Status::Todo,
    })
}
fn modify_task(total_task: &mut Vec<Task>) {
    for tasks in total_task {
        println!("{}", tasks);
      }
    let task_number = input_int("Enter the number of the task you want to modify: ");
    
}

fn main() {
    let mut total_task: Vec<Task> = Vec::new();
    let mut choice = 1;
    let mut task_id = 1;
    while choice >= 1 {
        println!("What do you want to do?: ");
        println!("Enter 1 to enter a task ");
        println!("Enter 2 to modify a task ");
        println!("Enter 3 to activate a pomodoro timer");
        println!("Enter 4 to delete a task");
        choice = input_int("Enter your choice: ");
        if choice == 1 {
            add_task(&mut total_task, task_id);
            task_id += 1;
        } else if choice == 2 {
            modify_task(&mut total_task);
        }
    }
}
