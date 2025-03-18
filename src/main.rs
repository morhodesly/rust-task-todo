use clap::Parser;
use clap::Subcommand;
use csv::ReaderBuilder;
use csv::WriterBuilder;
use serde_derive::{Deserialize, Serialize};
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
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: Uuid,
    title: String,
    due: String,
    status: String,
}

fn write_to_csv(task: Task) -> Result<(), Box<dyn std::error::Error>> {
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
        Commands::AllTask => {
            println!("Read All Task");
            let tasks = read_csv_file().unwrap();
            for task in tasks {
                println!("{:?}", task)
            }
        }
    }
}
