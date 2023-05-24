use crate::display_tasks;
use crate::file::get_tasks_from_file;
use dirs::config_dir;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn update_task(args: Vec<String>) {
    let tasks = get_tasks_from_file();
    let task_id = &args[2];
    let new_description = &args[3..].join(" ");
    let mut new_tasks: Vec<String> = Vec::new();

    for task in tasks {
        // example taskid. description. list
        let task = task.split(". ").collect::<Vec<&str>>();
        let id = task[0];
        let description = task[1];
        let list = task[2];

        if id == task_id {
            let new_task = format!("{}. {}. {}\n", id, new_description, list);
            new_tasks.push(new_task);
        } else {
            let new_task = format!("{}. {}. {}\n", id, description, list);
            new_tasks.push(new_task);
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

    display_tasks();
}
