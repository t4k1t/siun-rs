use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value as JSONValue;
use std::fs;
use std::path::PathBuf;

/// Representation of python types as serialized by siun
#[derive(Serialize, Deserialize, Debug)]
pub struct PyType {
    #[serde(rename = "py-type")]
    pub py_type: String,
    pub value: String,
}

/// Representation of siun state
#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub last_update: PyType,
    pub criteria_settings: JSONValue,
    pub thresholds: JSONValue,
    pub available_updates: Vec<String>,
    pub matched_criteria: JSONValue,
    pub state: PyType,
}

/// Read state from disk if available, otherwise return default state
pub fn load_state_from_disk() -> State {
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

    let state_path = xdg_state_home().join("state.json");
    let state_str = fs::read_to_string(&state_path);
    match state_str {
        Ok(state_str) => serde_json::from_str(&state_str).expect("failed to parse state JSON"),
        Err(_) => default_state,
    }
}

fn xdg_state_home() -> PathBuf {
    let state_path = std::env::var("XDG_STATE_HOME").unwrap_or(format!(
        "{}/.local/state/siun",
        std::env::var("HOME").unwrap_or_else(|_| "".to_string())
    ));
    PathBuf::from(&state_path)
}
