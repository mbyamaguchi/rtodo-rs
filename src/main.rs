use clap::Parser;

#[derive(Debug, Clone)]
struct Todo {
    text: String,
    date: String,
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
        date: String,
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
    let date = parts[1].to_string();
    let id = parts[2].to_string();
    let copmleted = parts[3].parse::<bool>().ok()?;

    Some(Todo {
        text,
        date,
        id,
        completed : completed,
    })
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { text, date } => {
            // Implementation for adding a todo
        }
        Commands::List => {
            // Implementation for listing todos
        }
        Commands::Complete { id } => {
            // Implementation for completing a todo
        }
        Commands::Delete { id } => {
            // Implementation for deleting a todo
        }
    }
}
