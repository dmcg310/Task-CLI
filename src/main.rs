use std::env;

fn get_tasks() {
    let tasks: Vec<String> = env::args().collect();
    display_tasks(tasks);
}

fn display_tasks(tasks: Vec<String>) {
    for task in tasks.iter().skip(1) {
        println!("{}", task);
    }
}

fn main() {
    get_tasks();
}
