use clap::Parser;
use clap::Subcommand;
use csv::ReaderBuilder;
use csv::WriterBuilder;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::fs::{OpenOptions, metadata};
use std::io::BufReader;
use uuid::Uuid;

#[derive(Parser)]
#[command(
    name = "CLI",
    version = "1.0",
    about = "A CLI for Creating, Updating and complete task"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CreateTask { title: String, due: String },
    AllTask,
    CompleteTask { id: String },
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: Uuid,
    title: String,
    due: String,
    status: String,
}

fn write_to_csv(task: Task) -> Result<(), Box<dyn Error>> {
    let file_path = "test.csv";
    let file_exists = metadata(file_path).is_ok();

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("test.csv")
        .unwrap();

    let mut writer = WriterBuilder::new()
        .has_headers(!file_exists)
        .from_writer(file);

    writer.serialize(task)?;
    writer.flush()?;

    Ok(())
}

fn write_tasks_to_csv(tasks: &[Task]) -> Result<(), Box<dyn Error>> {
    let mut writer = WriterBuilder::new().from_writer(File::create("test.csv")?);

    for task in tasks {
        writer.serialize(task)?;
    }

    writer.flush()?;
    Ok(())
}

fn read_csv_file() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let file_path = "test.csv";

    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().from_reader(BufReader::new(file));
    let mut records = Vec::new();
    for result in reader.deserialize() {
        let record = result?;
        records.push(record);
    }
    Ok(records)
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        // create new task to do
        Commands::CreateTask { title, due } => {
            println!("Test command {} {}", title, due);

            let task = Task {
                id: Uuid::new_v4(),
                title: title.to_string(),
                due: due.to_string(),
                status: "progress".to_string(),
            };

            if let Err(e) = write_to_csv(task) {
                eprint!("{}", e)
            }
        }
        // get all task
        Commands::AllTask => {
            println!("Read All Task");
            let tasks = read_csv_file().unwrap();
            for task in tasks {
                println!("{:?}", task)
            }
        }
        // Complete a task using id
        Commands::CompleteTask { id } => {
            println!("Complete Task {}", id);
            let mut tasks = read_csv_file().unwrap();
            if let Ok(search_id) = Uuid::parse_str(&id) {
                if let Some(task) = tasks.iter_mut().find(|task| task.id == search_id) {
                    println!("Found task: {:?}", task);
                    task.status = "Complete".to_string();
                    write_tasks_to_csv(&tasks).unwrap();
                } else {
                    println!("Task not found");
                }
            } else {
                println!("Invalid UUID format");
            }
        }
    }
}
