mod add;
mod delete;
mod file;
mod update;

use add::add_task;
use delete::delete_task;
use update::update_task;

use std::env;

fn display_tasks() {
    // tasks is defined above however we want updated tasks
    // todo: implement prettytable to display tasks
    // todo: update readme image to reflect prettytable
    let tasks = file::get_tasks_from_file();
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

fn main() {
    file::create_file();
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "a" => {
                if args.len() < 3 {
                    println!("Please provide a list name, and task description.");
                }
                add_task(args);
                display_tasks();
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
                         \nUse 'a <list-name> <task-description>' to add a task.
                         \n'u <task-id> <task-description>' to update a task.
                         \n'd <task-id>' to delete a task."
                );
                return;
            }
        }
    } else {
        display_tasks();
    }
}
