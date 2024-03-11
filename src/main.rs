use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JSONValue};
use std::collections::HashMap;
use std::fs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// CLI sub commands
#[derive(Subcommand)]
enum Commands {
    GetState {
        #[arg(short, long)]
        output_format: Option<OutputFormat>,
    },
}

/// Available output formats for CLI
#[derive(Clone, Eq, PartialEq, ValueEnum)]
enum OutputFormat {
    I3status,
    Fancy,
    Plain,
}

/// Representation of python types as serialized by siun
#[derive(Serialize, Deserialize, Debug)]
struct PyType {
    #[serde(rename = "py-type")]
    py_type: String,
    value: String,
}

/// Representation of siun state
#[derive(Serialize, Deserialize, Debug)]
struct State {
    last_update: PyType,
    criteria_settings: JSONValue,
    thresholds: JSONValue,
    available_updates: Vec<String>,
    matched_criteria: JSONValue,
    state: PyType,
}

/// Read state from disk if available, otherwise return default state
fn load_state_from_disk() -> State {
    let default_state = State {
        last_update: PyType {
            py_type: String::from(""),
            value: String::from(""),
        },
        criteria_settings: JSONValue::Null,
        thresholds: JSONValue::Null,
        available_updates: Vec::new(),
        matched_criteria: json!({}),
        state: PyType {
            py_type: String::from(""),
            value: String::from("UNKNOWN"),
        },
    };

    let state_str = fs::read_to_string("/tmp/siun-state.json");
    match state_str {
        Ok(state_str) => serde_json::from_str(&state_str).expect("failed to parse state JSON"),
        Err(_) => default_state,
    }
}

/// Format and print siun state for i3status
fn print_i3status(state: State) {
    let state_value: String = state.state.value;
    let shortened_criteria = state
        .matched_criteria
        .as_object()
        .expect("failed to read matched criteria")
        .keys()
        .map(|criterion| &criterion[0..2])
        .collect::<Vec<_>>()
        .join(",");

    let i3status_state_map = HashMap::from([
        ("OK", "Idle"),
        ("AVAILABLE_UPDATES", "Idle"),
        ("WARNING_UPDATES", "Warning"),
        ("CRITICAL_UPDATES", "Critical"),
        ("UNKNOWN", "Idle"),
    ]);
    let text_map = HashMap::from([
        ("OK", ""),
        ("AVAILABLE_UPDATES", ""),
        ("WARNING_UPDATES", &shortened_criteria),
        ("CRITICAL_UPDATES", &shortened_criteria),
        ("UNKNOWN", "…"),
    ]);

    let json_out = serde_json::json!({"icon": "narchive", "state": i3status_state_map.get(&state_value as &str), "text": text_map.get(&state_value as &str)});

    println!("{}", serde_json::to_string(&json_out).unwrap());
}

/// Format and print siun state as plain text
fn print_plain(state: State) {
    let text_map = HashMap::from([
        ("OK", "Ok"),
        ("AVAILABLE_UPDATES", "Updates available"),
        ("WARNING_UPDATES", "Updates recommended"),
        ("CRITICAL_UPDATES", "Updates required"),
    ]);

    let state_value: String = state.state.value;
    println!(
        "{}",
        text_map.get(&state_value as &str).unwrap_or(&"Unknown")
    );
}

/// Format and print siun state as colored text
fn print_fancy(state: State) {
    let text_map = HashMap::from([
        ("OK", "Ok"),
        ("AVAILABLE_UPDATES", "Updates available"),
        ("WARNING_UPDATES", "Updates recommended"),
        ("CRITICAL_UPDATES", "Updates required"),
    ]);
    let color_map = HashMap::from([
        ("OK", "\x1b[32m"),
        ("AVAILABLE_UPDATES", "\x1b[34m"),
        ("WARNING_UPDATES", "\x1b[33m"),
        ("CRITICAL_UPDATES", "\x1b[31m"),
    ]);

    let state_value: String = state.state.value;
    println!(
        "{}{}\x1b[0m",
        color_map.get(&state_value as &str).unwrap_or(&"\x1b[35m"),
        text_map.get(&state_value as &str).unwrap_or(&"Unknown")
    );
}

fn main() {
    let cli = Cli::parse();

    let state: State = load_state_from_disk();

    match &cli.command {
        Commands::GetState { output_format } => {
            match output_format {
                &Some(OutputFormat::I3status) => {
                    print_i3status(state);
                }
                &Some(OutputFormat::Fancy) => {
                    print_fancy(state);
                }
                &Some(OutputFormat::Plain) => {
                    print_plain(state);
                }
                None => {
                    print_plain(state);
                }
            };
        }
    }
}
