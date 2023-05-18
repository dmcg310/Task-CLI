use std::env;

struct Task {
    id: usize,
    description: String,
}

fn display_tasks(tasks: Vec<Task>) {
    println!("Tasks");
    println!("-----");

    for task in tasks.iter() {
        println!("{}. {}", task.id, task.description);
    }

    println!("-----");
}

fn add_task() {
    let mut tasks: Vec<Task> = vec![];
    let args: Vec<String> = env::args().collect();
    let description = &args[2];

    let task: Task = build_task(3, String::from(description));
    tasks.push(task);
    display_tasks(tasks);
}

fn build_task(id: usize, description: String) -> Task {
    Task { id, description }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // check if user wants to add task
    if &args.len() > &1 && &args[1] == "a" {
        add_task()
    } else {
        // TODO: get tasks from a file
        // boilerplate tasks for now
        let mut tasks: Vec<Task> = vec![];
        let task_1 = build_task(1, String::from("Clean"));
        let task_2 = build_task(2, String::from("Walk"));

        tasks.push(task_1);
        tasks.push(task_2);

        display_tasks(tasks)
    }
}
