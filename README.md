# Task-CLI

**Task-CLI** is a command-line interface (CLI) application written in Rust that helps you stay organized and manage your tasks efficiently. It provides a simple and intuitive way to create, update, and delete tasks directly from the command line.

## Features

- Add new tasks with a unique ID generated automatically.
- Update existing tasks by specifying the task ID and providing a new description.
- Delete tasks by their ID.
- Create multiple lists of tasks to seperate your tasks into different categories.
- View all tasks in a well-formatted and easy-to-read layout.
- Fast and lightweight, ensuring quick task management.

## Prerequisites

- Rust. Make sure you have Rust installed on your system. You can download it from the official Rust website: [https://www.rust-lang.org/](https://www.rust-lang.org/)

## Installation (Linux) (to be updated)

1. Clone the repository: `$ git clone https://github.com/dmcg310/Task-CLI.git`
2. Navigate to the project directory: `$ cd Task-CLI`
3. Build the project using Cargo: `$ cargo build --release`
4. Add the app(target/release/tcli) to your PATH and then you are good to go.

## Usage

- List tasks: `$ tlci`
- List tasks in a specific list: `$ tlci l <list-name>`
- Add a task: `$ tlci a <list-name> <task-description>`
- Update a task `$ tlci u <task-id> <new-task-description>`
- Delete a task `$ tlci d <task-id>`
