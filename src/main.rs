use colored::Colorize;
use std::io::{self, Write};
use std::process::Command;

struct TodoItem {
    id: u16,
    title: String,
    completed: bool,
}

pub fn clear_terminal_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
            .expect("failed to wait");
    };
}

fn add(list: &mut Vec<TodoItem>, title: &str) {
    let item = TodoItem {
        id: list.len() as u16 + 1,
        title: title.to_string(),
        completed: false,
    };
    list.push(item);
}

fn toggle(list: &mut [TodoItem], id: u16) {
    if let Some(item) = list.iter_mut().find(|item| item.id == id) {
        item.completed = !item.completed;
    } else {
        println!("Todo item with ID {} not found.", id);
    }
}

fn adjust_ids(list: &mut [TodoItem]) {
    for (index, item) in list.iter_mut().enumerate() {
        item.id = index as u16 + 1;
    }
}

fn remove(list: &mut Vec<TodoItem>, id: u16) {
    if let Some(index) = list.iter().position(|item| item.id == id) {
        list.remove(index);
        adjust_ids(list);
    } else {
        println!("Todo item with ID {} not found.", id);
    }
}

fn display_todo(list: &Vec<TodoItem>) {
    for i in list {
        if i.completed {
            let s = format!("{} {:03}: {}", "󰄲".blue(), i.id, i.title)
                .strikethrough()
                .bright_black();
            println!("{}", s);
        } else {
            let s = format!("{} {:03}: {}", "󰄱".red(), i.id, i.title);
            println!("{}", s);
        }
    }
}

fn read_command() -> String {
    print!("> ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn parse_command(input: &str, list: &mut Vec<TodoItem>) {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    match parts.as_slice() {
        ["add", title] => add(list, title),
        ["remove", id] => {
            if let Ok(id) = id.parse::<u16>() {
                remove(list, id)
            } else {
                println!("Invalid ID format.")
            }
        }
        ["toggle", id] => {
            if let Ok(id) = id.parse::<u16>() {
                toggle(list, id)
            } else {
                println!("Invalid ID format.")
            }
        }
        _ => println!("Invalid command."),
    }
}

fn main() {
    let mut todo_list: Vec<TodoItem> = Vec::new();
    clear_terminal_screen();
    add(
        &mut todo_list,
        &format!("{}", "Type \"help\" for help".yellow()),
    );
    display_todo(&todo_list);

    loop {
        let command = read_command();
        clear_terminal_screen();
        match command.as_str() {
            "quit" | "q" => break,
            "help" | "h" => {
                println!("{}", "🐧 PenList\n────────────────────────────".blue());
                println!(
"  help: prints this message.
  add <title>: adds an item to the list.
  remove <id>: remove an item from the list.
  toggle <id>: toggle an item to checked and unchecked such as 󰄱 and 󰄲
  quit: quit from application.

  Aliases: help: (h); add: (a); remove: (rm); toggle: (done, dn); quit: (q);
"
                );
            },
            _ => {
                parse_command(&command, &mut todo_list);
                display_todo(&todo_list);
            }
        }
    }
}
