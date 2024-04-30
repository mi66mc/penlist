use colored::Colorize;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct TodoItem {
    id: u16,
    title: String,
    completed: bool
}

fn save_todo_list(list: &[TodoItem], filename: &str) -> std::io::Result<()> {
    let serialized = serde_json::to_string(&list)?;
    fs::write(filename, serialized)?;
    Ok(())
}

fn load_todo_list(filename: &str) -> std::io::Result<Vec<TodoItem>> {
    let content = fs::read_to_string(filename)?;
    let deserialized: Vec<TodoItem> = serde_json::from_str(&content)?;
    Ok(deserialized)
}

pub fn clear_terminal_screen() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        std::process::Command::new("clear")
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
        completed: false
    };
    list.push(item);
}

fn toggle(list: &mut Vec<TodoItem>, id: u16) {
    if let Some(item) = list.iter_mut().find(|item| item.id == id) {
        item.completed = !item.completed;
    } else {
        println!("{}", format!("Todo item with ID {} not found.", id).bright_red());
    }
}

fn adjust_ids(list: &mut Vec<TodoItem>) {
    for (index, item) in list.iter_mut().enumerate() {
        item.id = index as u16 + 1;
    }
}

fn remove(list: &mut Vec<TodoItem>, id: u16) {
    if let Some(index) = list.iter().position(|item| item.id == id) {
        list.remove(index);
        adjust_ids(list);
    } else {
        println!("{}", format!("Todo item with ID {} not found.", id).bright_red());
    }
}

fn display_todo(list: &Vec<TodoItem>) {
    println!("{}", "────────────────────────────\n🐧 PenList\n────────────────────────────".blue().bold());
    for i in list {
        if i.completed {
            let s = format!("{} {:03}: {}", "󰄲".blue(), i.id, i.title).strikethrough().bright_black();
            println!("{}", s);
        } else {
            let s = format!("{} {:03}: {}", "󰄱".red(), i.id, i.title);
            println!("{}", s);
        }
    }
    
}

fn read_command(list: &Vec<TodoItem>) -> String {
    println!("{}", progress_bar(list));
    print!("> ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn progress_bar(list: &Vec<TodoItem>) -> String {
    let total_items = list.len();
    let completed_items = list.iter().filter(|item| item.completed).count();
    let progress_bar_length = 28;
    let progress = if total_items > 0 {
        completed_items as f32 / total_items as f32
    } else {
        0.0
    };
    let progress_bar_filled = (progress * progress_bar_length as f32) as usize;
    let progress_bar_empty = progress_bar_length - progress_bar_filled;
    let progress_bar = format!(
        "{}{}",
        "─".repeat(progress_bar_filled).bright_green(),
        "─".repeat(progress_bar_empty).bright_black()
    );
    format!("{} {}/{}", progress_bar, completed_items, total_items)
}

fn parse_command(input: &str, list: &mut Vec<TodoItem>) {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    match parts.as_slice() {
        ["add", title] | ["a", title] => add(list, title),
        ["remove", id] | ["rm", id] => {
            if let Ok(id) = id.parse::<u16>() {
                remove(list, id)
            } else {
                println!("{}", "Invalid ID format.".red())
            }
        }
        ["toggle", id] | ["done", id] | ["dn", id] => {
            if let Ok(id) = id.parse::<u16>() {
                toggle(list, id)
            } else {
                println!("{}", "Invalid ID format.".red())
            }
        }
        ["save", filename] => {
            if let Err(err) = save_todo_list(list, filename) {
                println!("{}", format!("Error saving todo list: {}", err).bright_red());
            } else {
                println!("{}", "Todo list saved successfully.".bright_green());
            }
        }
        ["load", filename] => {
            match load_todo_list(filename) {
                Ok(loaded_list) => {
                    *list = loaded_list;
                    println!("{}", "Todo list loaded successfully.".bright_green());
                }
                Err(err) => println!("{}", format!("Error loading todo list: {}", err).red()),
            }
        }
        _ => println!("{}", "Invalid command.".bright_red()),
    }
}

fn main() {
    let mut todo_list: Vec<TodoItem> = Vec::new();
    clear_terminal_screen();
    add(&mut todo_list, &format!("{}", "Type \"help\" for help".yellow()));
    display_todo(&todo_list);
    
    loop {
        let command = read_command(&todo_list);
        clear_terminal_screen();
        match command.as_str() {
            "quit" | "q" => break,
            "help" | "h" => {
                println!("{}", "────────────────────────────\n🐧 PenList\n────────────────────────────".blue());
                println!("{}",
"  help: prints this message.
  add <title>: adds an item to the list.
  remove <id>: remove an item from the list.
  toggle <id>: toggle an item to checked and unchecked such as 󰄱 and 󰄲
  quit: quit from application.
  save <filename>: save your todo list into a file.
  load <filename>: load your todo list from a file.

  Aliases: help: (h); add: (a); remove: (rm); toggle: (done, dn); quit: (q);
".bright_black()
                );
            },
            _ => {
                parse_command(&command, &mut todo_list);
                display_todo(&todo_list);
            }
        }
    }
}