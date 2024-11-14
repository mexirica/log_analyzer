use std::collections::HashMap;
use clap::{Parser, Subcommand, ValueEnum};
use comfy_table::{Table, presets::UTF8_FULL};
use chrono::{NaiveDateTime};
use regex::Regex;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use lazy_static::lazy_static;
use rayon::prelude::*;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Caminho para o arquivo de log
    #[arg(short, long)]
    log_path: String,

    /// Caminho para salvar o output (opcional)
    #[arg(short, long)]
    output: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze the log file with filters
    Analyze {
        /// Start datetime for filter (format dd/mm/yyyy hh:mm)
        #[arg(short = 's', long)]
        start_time: Option<String>,

        /// End datetime for filter(format dd/mm/yyyy hh:mm)
        #[arg(short = 'e', long)]
        end_time: Option<String>,

        #[arg(short = 'l', long)]
        log_level: Option<LogLevel>,

        /// Keyword para filtragem
        #[arg(short = 'k', long)]
        keyword: Option<String>,
    },

    /// Shows an overview of the log file
    Overview,
}

lazy_static! {
    static ref LOG_REGEX: Regex = Regex::new(r"(\d{2}/\d{2}/\d{4} \d{2}:\d{2});\s*(\w+);\s*(.+)").unwrap();
}

#[derive(ValueEnum, Clone, Debug)]
enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: NaiveDateTime,
    pub log_type: String,
    pub message: String,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let log_entries = load_logs(&args.log_path)?;

    match args.command {
        Commands::Analyze {
            start_time,
            end_time,
            log_level,
            keyword,
        } => {
            let filtered_logs = filter_logs(&log_entries, start_time, end_time, log_level, keyword);
            display_logs(filtered_logs, args.output);
        },
        Commands::Overview => {
            let summary = summarize_logs(&log_entries);
            display_overview(summary);
        }
    }
    Ok(())
}

fn load_logs(log_path: &str) -> io::Result<Vec<LogEntry>> {
    let file = File::open(log_path)?;
    let reader = io::BufReader::new(file);
    let mut logs = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Some(log_entry) = parse_log_line(&line) {
            logs.push(log_entry);
        }
    }

    Ok(logs)
}

fn parse_log_line(line: &str) -> Option<LogEntry> {
    if let Some(caps) = LOG_REGEX.captures(line) {
        let timestamp = NaiveDateTime::parse_from_str(&caps[1], "%d/%m/%Y %H:%M").ok()?;
        let log_type = caps[2].to_string();
        let message = caps[3].to_string();

        Some(LogEntry { timestamp, log_type, message })
    } else {
        None
    }
}

fn filter_logs(
    logs: &[LogEntry],
    start_time: Option<String>,
    end_time: Option<String>,
    log_level: Option<LogLevel>,
    keyword: Option<String>,
) -> Vec<&LogEntry> {
    let start_dt = start_time.and_then(|s| NaiveDateTime::parse_from_str(&s, "%d/%m/%Y %H:%M").ok());
    let end_dt = end_time.and_then(|e| NaiveDateTime::parse_from_str(&e, "%d/%m/%Y %H:%M").ok());
    let level_str = log_level.map(|level| format!("{:?}", level).to_uppercase());
    let keyword_str = keyword.unwrap_or_default();

    let filter = |log: &&LogEntry| {
        let mut valid = true;

        if let Some(start) = start_dt {
            valid &= log.timestamp >= start;
        }
        if let Some(end) = end_dt {
            valid &= log.timestamp <= end;
        }
        if let Some(level) = &level_str {
            valid &= log.log_type == *level;
        }
        if !keyword_str.is_empty() && !log.message.contains(&keyword_str) {
            valid &= false;
        }

        valid
    };

    if logs.len() > 1000 {  // Ajuste esse valor conforme necessário
        logs.par_iter()
            .filter(filter)
            .collect()
    } else {
        logs.iter()
            .filter(filter)
            .collect()
    }
}


fn display_logs(logs: Vec<&LogEntry>, output: Option<String>) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["DateTime", "Level", "Message"]);

    for log in logs {
        table.add_row(vec![
            log.timestamp.to_string(),
            log.log_type.clone(),
            log.message.clone(),
        ]);
    }

    if let Some(out) = output {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(out)
            .expect("Error to open output file");

        writeln!(file, "{table}").expect("Error to write to file");
    } else {
        println!("{table}");
        println!("Number of results: {}", table.row_count());
    }
}

fn summarize_logs(logs: &[LogEntry]) -> HashMap<String, usize> {
    let mut summary = HashMap::new();

    for log in logs {
        let counter = summary.entry(log.log_type.clone()).or_insert(0);
        *counter += 1;
    }

    summary
}

fn display_overview(summary: HashMap<String, usize>) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["Log Level", "Count"]);

    for (level, count) in summary {
        table.add_row(vec![level, count.to_string()]);
    }

    println!("{table}");
}
