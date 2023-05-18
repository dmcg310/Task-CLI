use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

struct Task {
    id: u16,
    description: String,
}

fn build_task(id: u16, description: String) -> Task {
    Task { id, description }
}

fn display_tasks(tasks: Vec<String>) {
    if tasks.len() == 0 {
        // todo fix bug
        println!("No tasks found. Use 'cargo run a <task-description>'")
    } else {
        println!("Task-CLI");
        println!("-----");

        for task in tasks.iter() {
            println!("{}", task);
        }

        println!("-----");
    }
}

fn add_task() {
    let args: Vec<String> = env::args().collect();
    let tasks: Vec<String> = get_tasks_from_file();
    let id = tasks.len() as u16 + 1;
    let description = &args[2..].join(" ");

    let task: Task = build_task(id, String::from(description));
    let mut file = OpenOptions::new()
        .append(true)
        .open("tasks.txt")
        .expect("Unable to open file");

    if let Err(err) = writeln!(file, "{}. {}", task.id, task.description) {
        eprintln!("Failed to write to file: {}", err);
    } else {
        let new_tasks = get_tasks_from_file();
        display_tasks(new_tasks)
    }
}

fn update_task() {
    let args: Vec<String> = env::args().collect();
    let task_id = &args[2];
    let new_description = &args[3..].join(" ");
    let tasks = get_tasks_from_file();
    let mut new_tasks: Vec<String> = Vec::new();

    for task in tasks {
        let mut task_id_and_description = task.split(". ");
        let id = task_id_and_description.next().unwrap();

        if id == task_id {
            let new_task = format!("{}. {}", id, new_description);
            new_tasks.push(new_task);
        } else {
            new_tasks.push(task);
        }
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("tasks.txt")
        .expect("Unable to open file");

    for task in new_tasks {
        if let Err(err) = writeln!(file, "{}", task) {
            eprintln!("Failed to write to file: {}", err);
        }
    }
}

fn file_creation() {
    let file_name = "tasks.txt";

    if std::path::Path::new(&file_name).exists() {
        return;
    } else {
        match File::create(&file_name) {
            Ok(_file) => {
                return;
            }
            Err(err) => eprintln!("Failed to create file: {}", err),
        }
    }
}

fn get_tasks_from_file() -> Vec<String> {
    let file = File::open("tasks.txt").expect("File not found.");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|x| x.expect("Could not parse line."))
        .collect()
}

fn main() {
    file_creation();

    let tasks: Vec<String> = get_tasks_from_file();
    let args: Vec<String> = env::args().collect();

    // check if user wants to add a task
    if &args.len() > &1 && &args[1] == "a" {
        add_task()
        // check if user wants to update a task
    } else if &args.len() > &1 && &args[1] == "u" {
        update_task()
    } else {
        display_tasks(tasks)
    }
}
