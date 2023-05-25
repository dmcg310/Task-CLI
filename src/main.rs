extern crate prettytable;

mod add;
mod delete;
mod display;
mod file;
mod update;

use add::add_task;
use delete::delete_task;
use display::{display_tasks, display_tasks_list};
use update::update_task;

use std::env;

fn main() {
    file::create_file();
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "a" => {
                if args.len() < 3 {
                    println!("Please provide a list name, and task description.");
                }
                add_task();
                display_tasks_list(args);
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
            "l" => {
                if args.len() < 3 {
                    println!("Please provide a list name.");
                }
                display_tasks_list(args);
            }
            _ => {
                println!(
                    "Invalid command.
                         \nRun 'tcli' to display all tasks.
                         \n'a <list-name> <task-description>' to add a task.
                         \n'u <task-id> <task-description>' to update a task.
                         \n'd <task-id>' to delete a task.
                         \n'l <list-name>' to display tasks in a list."
                );
                return;
            }
        }
    } else {
        display_tasks();
    }
}
