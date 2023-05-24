use crate::file::get_tasks_from_file;
use dirs::config_dir;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn delete_task(args: Vec<String>) {
    let tasks = get_tasks_from_file();
    let task_id = &args[2];

    let mut new_tasks: Vec<String> = Vec::new();

    for task in tasks {
        let mut task_id_and_description = task.split(". ");
        let id = task_id_and_description.next().unwrap();

        if id != task_id {
            new_tasks.push(task.to_string());
        }
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(config_dir().unwrap().join("tcli.txt"))
        .expect("Unable to open file");

    for task in new_tasks {
        if let Err(err) = writeln!(file, "{}", task) {
            eprintln!("Failed to write to file: {}", err);
        }
    }

    println!("Task deleted!");
}
