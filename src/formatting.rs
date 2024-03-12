use crate::state::State;
use clap::ValueEnum;
use std::collections::HashMap;

/// Available output formats for CLI
#[derive(Clone, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    I3status,
    Fancy,
    Plain,
}

/// Format and print siun state for i3status
pub fn print_i3status(state: State) {
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
pub fn print_plain(state: State) {
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
pub fn print_fancy(state: State) {
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
