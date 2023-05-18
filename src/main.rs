use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

struct Task {
    description: String,
}

fn build_task(description: String) -> Task {
    Task { description }
}

fn display_tasks(tasks: Vec<String>) {
    let mut task_id = 1;

    println!("Tasks");
    println!("-----");

    for task in tasks.iter() {
        println!("{}. {}", task_id, task);
        task_id += 1;
    }

    println!("-----");
}

fn add_task() {
    let args: Vec<String> = env::args().collect();
    let description = &args[2];

    let task: Task = build_task(String::from(description));
    let mut file = OpenOptions::new()
        .append(true)
        .open("tasks.txt")
        .expect("Unable to open file");

    if let Err(err) = writeln!(file, "{}", task.description) {
        eprintln!("Failed to write to file: {}", err);
    } else {
        println!("Task added successfully");
    }
}

fn file_creation() {
    let file_name = "tasks.txt";

    if std::path::Path::new(&file_name).exists() {
        println!("File exists")
    } else {
        match File::create(&file_name) {
            Ok(_file) => {
                println!("File created")
            }
            Err(err) => eprintln!("Failed to create file: {}", err),
        }
    }
}

fn tasks_from_file() -> Vec<String> {
    let file = File::open("tasks.txt").expect("File not found.");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|x| x.expect("Could not parse line."))
        .collect()
}

fn main() {
    file_creation();

    let tasks: Vec<String> = tasks_from_file();
    let args: Vec<String> = env::args().collect();

    // check if user wants to add task
    if &args.len() > &1 && &args[1] == "a" {
        add_task()
    } else {
        display_tasks(tasks)
    }
}
