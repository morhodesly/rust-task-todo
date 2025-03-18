# Task Management CLI

## Overview
This is a simple Command-Line Interface (CLI) tool written in Rust for managing tasks. The CLI allows users to create new tasks and list all existing tasks stored in a CSV file.

## Features
- Create new tasks with a title and due date.
- View all tasks stored in a CSV file.

## Prerequisites
Ensure you have the following installed on your system:
- [Rust](https://www.rust-lang.org/) (with `cargo` package manager)

## Installation
Clone the repository and navigate into the project directory:
```sh
git clone <repository-url>
cd <repository-folder>
```
Build the project:
```sh
cargo build --release
```
Run the CLI:
```sh
cargo run -- --help
```

## Usage
### Create a New Task
To create a new task, use the following command:
```sh
cargo run -- create-task "Task Title" "YYYY-MM-DD"
```
Example:
```sh
cargo run -- create-task "Finish report" "2025-03-20"
```

### View All Tasks
To view all tasks stored in the CSV file:
```sh
cargo run -- all-task
```

## File Storage
- Tasks are stored in `test.csv` in CSV format.
- Each task has the following fields:
  - `id`: A unique identifier (UUID) for the task.
  - `title`: The task's title.
  - `due`: The due date of the task.
  - `status`: The status of the task (default: "progress").

## Dependencies
The project uses the following Rust crates:
- `clap` for CLI argument parsing
- `csv` for handling CSV file operations
- `serde` and `serde_derive` for serialization and deserialization
- `uuid` for generating unique task IDs

## Contributing
Feel free to fork the repository and submit pull requests to enhance the functionality.

## License
This project is licensed under the MIT License.

## Author
Mohamed Rhodesly

