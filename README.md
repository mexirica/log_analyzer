This is a **CLI (Command Line Interface)** tool developed in **Rust** for log analysis. It allows filtering logs by level, date, and performing keyword searches. The application is designed to be simple, efficient, and fast, leveraging Rust's compiler optimizations.

## Features

- **Level Filtering**: Filter logs based on their level (e.g., ERROR, WARNING, INFO).
- **Keyword Search**: Search for specific words or phrases in the logs.
- **Date Filtering**: Filter logs by a specific date or date range.
- **Performance**: The application is optimized to handle large log datasets efficiently.

## Installation

### Prerequisites

Make sure you have **Rust** installed on your machine. If you don't have it yet, you can install it from the [official Rust website](https://www.rust-lang.org/tools/install).

### Installation Steps

1. Clone this repository to your local machine:

```bash
   git clone https://github.com/your-username/log_analyzer.git
   ```
2. Navigate to the project directory:

```bash
   cd log_analyzer
   ```
3. Build the project with the following command:

```bash
    cargo build --release
  ```
4. The binary will be generated in target/release/log_analyzer. You can move this binary to any directory in your PATH for easier access.

### How to install

```bash
  cargo install --path .
```
### How to use

1. To search
```bash
log_analyzer --log-path <-path> analyze <-k> <-s> <-e> <-l>
```

2. To an overview
```bash
log_analyzer --log-path <-path> overview
```

### Examples

<img width="724" alt="Captura de Tela 2024-11-15 às 18 24 24" src="https://github.com/user-attachments/assets/ef775e01-1b4a-47bd-ac94-c5a40b95cced">
<img width="1145" alt="Captura de Tela 2024-11-15 às 18 24 03" src="https://github.com/user-attachments/assets/bd266f14-a7e4-4b28-9d5e-f5dc6af014c9">


##Next updates

1. Read a json file.
2. Choose the output (Console,csv,json,.log)


