use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use uuid::Uuid;

struct Task {
    id: String,
    description: String,
}

fn generate_uuid() -> String {
    let tasks = get_tasks_from_file();

    let uuid = Uuid::new_v4();
    let mut digits_only = String::new();

    for c in uuid.to_string().chars() {
        if c.is_digit(10) {
            digits_only.push(c);
            if digits_only.len() == 3 {
                break;
            }
        }
    }

    // check if any current uuids are the same as the generated one
    for task in tasks {
        let mut task_id_and_description = task.split(". ");
        let id = task_id_and_description.next().unwrap();
        if id == digits_only {
            generate_uuid();
            break;
        }
    }

    digits_only
}
fn build_task(id: String, description: String) -> Task {
    Task { id, description }
}

fn display_tasks(tasks: Vec<String>) {
    let color_blue = "\x1B[34m";
    let color_green = "\x1b[32m";
    let color_default = "\x1B[0m";

    if tasks.len() == 0 {
        println!("No tasks found. Use 'cargo run a <task-description>'")
    } else {
        let header = format!("{} Task-CLI {}", color_blue, color_default);
        println!("{}", header);

        let borders = format!("{} ---------- {} ", color_green, color_default);
        println!("{}", borders);

        for task in tasks.iter() {
            println!("{}", task);
        }

        println!("{}", borders);
    }
}

fn add_task() {
    let args: Vec<String> = env::args().collect();
    let id = generate_uuid();
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

fn delete_task() {
    let args: Vec<String> = env::args().collect();
    let task_id = &args[2];

    let tasks = get_tasks_from_file();
    let mut new_tasks: Vec<String> = Vec::new();

    for task in tasks {
        let mut task_id_and_description = task.split(". ");
        let id = task_id_and_description.next().unwrap();

        if id != task_id {
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

    if &args.len() > &1 && &args[1] == "a" {
        // check if user wants to add a task
        add_task();
    } else if &args.len() > &1 && &args[1] == "u" {
        // check if user wants to update a task
        update_task();
    } else if &args.len() > &1 && &args[1] == "d" {
        // check if user wants to delete a task
        delete_task();
    } else {
        display_tasks(tasks);
    }
}
