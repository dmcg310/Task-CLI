use dirs::config_dir;
use rand::Rng;
use std::fs::OpenOptions;
use std::io::prelude::*;

use crate::file::get_tasks_from_file;

pub struct Task {
    id: u32,
    description: String,
    list: String,
}

pub fn generate_id() -> u32 {
    let mut rng = rand::thread_rng();
    let tasks: Vec<String> = get_tasks_from_file();

    loop {
        let id = rng.gen_range(10..=99);
        let id = id.to_string();

        if !tasks.contains(&id) {
            return id.parse::<u32>().unwrap();
        }
    }
}

pub fn build_task(id: u32, description: String, list: String) -> Task {
    Task {
        id,
        description,
        list,
    }
}

pub fn add_task(args: Vec<String>) -> Vec<String> {
    let list = &args[2];
    let description = &args[3..].join(" ");
    let id = generate_id();

    let task: Task = build_task(id, String::from(description), String::from(list));
    let mut file = OpenOptions::new()
        .append(true)
        .open(config_dir().unwrap().join("tcli.txt"))
        .expect("Unable to open file");

    let new_task = format!("{}. {}. {}\n", task.id, task.description, task.list);
    file.write_all(new_task.as_bytes())
        .expect("Unable to write to file");

    let mut tasks: Vec<String> = Vec::new();
    tasks.push(new_task);

    tasks
}
