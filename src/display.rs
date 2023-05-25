use crate::file;
use prettytable::{color, Attr, Cell, Row, Table};

pub fn display_tasks() {
    let tasks: Vec<String> = file::get_tasks_from_file();

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("ID")
            .with_style(Attr::ForegroundColor(color::WHITE))
            .with_style(Attr::Bold),
        Cell::new("Description")
            .with_style(Attr::ForegroundColor(color::WHITE))
            .with_style(Attr::Bold),
        Cell::new("List")
            .with_style(Attr::ForegroundColor(color::WHITE))
            .with_style(Attr::Bold),
    ]));

    for task in tasks {
        let task = task.split(". ").collect::<Vec<&str>>();
        let id = task[0];
        let description = task[1];
        let list = task[2];

        table.add_row(Row::new(vec![
            Cell::new(id)
                .with_style(Attr::ForegroundColor(color::BLUE))
                .with_style(Attr::Italic(true)),
            Cell::new(description),
            Cell::new(list),
        ]));
    }

    table.printstd();
}

pub fn display_tasks_list(args: Vec<String>) {
    let tasks: Vec<String> = file::get_tasks_from_file();
    let list = &args[2];

    let mut list_tasks = vec![];
    for task in tasks {
        let task_arr = task.split(". ").collect::<Vec<&str>>();
        let list_name = task_arr[2];

        if list_name == list {
            list_tasks.push(task);
        }
    }
    if list_tasks.len() == 0 {
        println!(
            "No tasks in list `{}`. Run `tcli a <list-name> <list-description>` to add a task.",
            list
        );
        return;
    }

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("ID")
            .with_style(Attr::ForegroundColor(color::WHITE))
            .with_style(Attr::Bold),
        Cell::new("Description")
            .with_style(Attr::ForegroundColor(color::WHITE))
            .with_style(Attr::Bold),
        Cell::new("List")
            .with_style(Attr::ForegroundColor(color::WHITE))
            .with_style(Attr::Bold),
    ]));

    for task in list_tasks {
        let task = task.split(". ").collect::<Vec<&str>>();
        let id = task[0];
        let description = task[1];
        let list = task[2];

        table.add_row(Row::new(vec![
            Cell::new(id)
                .with_style(Attr::ForegroundColor(color::BLUE))
                .with_style(Attr::Italic(true)),
            Cell::new(description),
            Cell::new(list),
        ]));
    }

    table.printstd();
}
