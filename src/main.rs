use rand::Rng;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

const FILE_NAME: &str = "/home/dmcg310/tasks.txt";

struct Task {
    id: u32,
    description: String,
}

fn generate_id() -> u32 {
    let tasks = get_tasks_from_file();

    let mut rng = rand::thread_rng();
    loop {
        let new_id: u32 = rng.gen_range(10..=99);

        // check if any current IDs are the same as the generated one
        if !tasks.iter().any(|task| {
            let mut task_id_and_description = task.split(". ");
            let id = task_id_and_description
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            id == new_id
        }) {
            return new_id;
        }
    }
}

fn build_task(id: u32, description: String) -> Task {
    Task { id, description }
}

fn display_tasks(tasks: Vec<String>) {
    let color_blue = "\x1B[34m";
    let color_green = "\x1b[32m";
    let color_default = "\x1B[0m";

    if tasks.len() == 0 {
        println!("No tasks found. Use 'tcli a <task-description>'")
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

fn add_task(args: Vec<String>) {
    let description = &args[2..].join(" ");
    let id = generate_id();

    let task: Task = build_task(id, String::from(description));
    let mut file = OpenOptions::new()
        .append(true)
        .open(FILE_NAME)
        .expect("Unable to open file");

    if let Err(err) = writeln!(file, "{}. {}", task.id, task.description) {
        eprintln!("Failed to write to file: {}", err);
    } else {
        let new_tasks = get_tasks_from_file();
        display_tasks(new_tasks)
    }
}

fn update_task(args: Vec<String>) {
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
        .open(FILE_NAME)
        .expect("Unable to open file");

    for task in new_tasks {
        if let Err(err) = writeln!(file, "{}", task) {
            eprintln!("Failed to write to file: {}", err);
        }
    }

    let updated_tasks = get_tasks_from_file();
    display_tasks(updated_tasks);
}

fn delete_task(args: Vec<String>) {
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
        .open(FILE_NAME)
        .expect("Unable to open file");

    for task in new_tasks {
        if let Err(err) = writeln!(file, "{}", task) {
            eprintln!("Failed to write to file: {}", err);
        }
    }

    println!("Task deleted!");
}

fn file_creation() {
    if std::path::Path::new(&FILE_NAME).exists() {
        return;
    } else {
        match File::create(&FILE_NAME) {
            Ok(_file) => {
                return;
            }
            Err(err) => eprintln!("Failed to create file: {}", err),
        }
    }
}

fn get_tasks_from_file() -> Vec<String> {
    let file = File::open(FILE_NAME).expect("File not found.");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|x| x.expect("Could not parse line."))
        .collect()
}

fn main() {
    file_creation();

    let tasks = get_tasks_from_file();
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "a" => {
                if args.len() < 3 {
                    println!("Please provide a task description.");
                }
                add_task(args);
            }
            "u" => {
                if args.len() < 4 {
                    println!("Please provide a task ID and a new description.");
                }
                update_task(args);
            }
            "d" => {
                if args.len() < 3 {
                    println!("Please provide a task ID to delete.");
                }
                delete_task(args);
            }
            _ => {
                println!(
                    "Invalid command.
                         \nUse 'a <task-description>' to add a task.
                         \n'u <task-id> <task-description>' to update a task.
                         \n'd <task-id>' to delete a task."
                );
                return;
            }
        }
    } else {
        display_tasks(tasks)
    }
}
