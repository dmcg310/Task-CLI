# Task-CLI
![image](https://github.com/dmcg310/Task-CLI/assets/120114728/92a2d329-89f6-4738-b7db-43d130c6d01d)


**Task-CLI** is a command-line interface (CLI) application written in Rust that helps you stay organized and manage your tasks efficiently. It provides a simple and intuitive way to create, update, and delete tasks directly from the command line.

## Features

- Add new tasks with a unique ID generated automatically.
- Update existing tasks by specifying the task ID and providing a new description.
- Delete tasks by their ID.
- View all tasks in a well-formatted and easy-to-read layout.
- Fast and lightweight, ensuring quick task management.

## Prerequisites

- Rust. Make sure you have Rust installed on your system. You can download it from the official Rust website: [https://www.rust-lang.org/](https://www.rust-lang.org/)

## Installation (Linux)

1. Clone the repository: ``$ git clone https://github.com/dmcg310/Task-CLI.git``
2. Navigate to the project directory: ``$ cd Task-CLI``
3. Change the file paths in the src/main.rs file to your own username.
4. Build the project using Cargo: ``$ cargo build --release``
5. Add the app(target/release/tcli) to your PATH and then you are good to go.

## Usage

- List tasks: ``$ tlci``
- Add a task: ``$ tlci a <task-description>``
- Update a task ``$ tlci u <task-id> <new-task-description>``
- Delete a task ``$ tlci d <task-id>``

## Other Notes

This app creates a txt file called 'tasks' in your home directory and if it is removed; your tasks will also be removed.
