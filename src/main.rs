use clap::Parser;
// use dirs::home_dir;
use std::fs::{OpenOptions, File};
use std::io::{BufRead, BufReader, Write};
use uuid::Uuid;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
struct Todo {
    text: String,
    date: NaiveDate,
    id: String,
    completed: bool,
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    Add {
        text: String,
        date: NaiveDate,
    },
    List,
    Complete {
        id: String,
    },
    Delete {
        id: String,
    }
}



fn parse_todo_line(line: &str) -> Option<Todo> {
    let parts: Vec<&str> = line.split(' ').collect();
    if parts.len() < 4 {
        return None;
    }
    let text = parts[0].to_string();
    let date = parts[1].parse::<NaiveDate>().ok()?;
    let id = parts[2].to_string();
    let completed = parts[3].parse::<bool>().ok()?;

    Some(Todo {
        text,
        date,
        id,
        completed,
    })
}

fn show_all_todos(file: &File) {
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(todo) = parse_todo_line(&line) {
                println!("ID: {}, Text: {}, Date: {}, Completed: {}", todo.id, todo.text, todo.date, todo.completed);
            }
        }
    }
}

fn mark_complete(file: &mut File, id: &str) {
    let mut todos = Vec::new();
    let reader = BufReader::new(&mut *file);
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(mut todo) = parse_todo_line(&line) {
                if todo.id == id {
                    todo.completed = true;
                }
                todos.push(todo);
            }
        }
    }

    file.set_len(0).expect("Failed to clear file");
    for todo in todos {
        writeln!(file, "{} {} {} {}", todo.text, todo.date, todo.id, todo.completed).expect("Failed to write to file");
    }
}

fn add_todo(file: &mut File, text: String, date: NaiveDate) {
    let new_todo = Todo {
        text,
        date,
        id: Uuid::new_v4().to_string(),
        completed: false,
    };
    writeln!(file, "{} {} {} {}", new_todo.text, new_todo.date, new_todo.id, new_todo.completed).expect("Failed to write to file");
}

fn delete_todo(file: &mut File, id: &str) {
    let mut todos = Vec::new();
    let reader = BufReader::new(&mut *file);
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(todo) = parse_todo_line(&line) {
                if todo.id != id {
                    todos.push(todo);
                }
            }
        }
    }

    file.set_len(0).expect("Failed to clear file");
    for todo in todos {
        writeln!(file, "{} {} {} {}", todo.text, todo.date, todo.id, todo.completed).expect("Failed to write to file");
    }
}

fn main() {

    const TODO_FILE: &str = "todo.txt";
    // let todo_file = format!("{}/.local/share/{}", home_dir().unwrap().display(), TODO_FILE);
    let todo_file = format!("./{}", TODO_FILE);


    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&todo_file)
        .expect("Failed to open todo file");


    let cli = Cli::parse();
    match cli.command {
        Commands::Add { text, date } => {
            // Implementation for adding a todo
            let new_todo = Todo {
                text,
                date,
                id: Uuid::new_v4().to_string(),
                completed: false,
            };
            add_todo(&mut file, new_todo.text, new_todo.date);
        }
        Commands::List => {
            // Implementation for listing todos
            show_all_todos(&file);
        }
        Commands::Complete { id } => {
            // Implementation for completing a todo
            mark_complete(&mut file, &id);
        }
        Commands::Delete { id } => {
            // Implementation for deleting a todo
            delete_todo(&mut file, &id);
        }
    }
}
