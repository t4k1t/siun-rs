use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value as JSONValue;
use std::fs;

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

    let state_str = fs::read_to_string("/tmp/siun-state.json");
    match state_str {
        Ok(state_str) => serde_json::from_str(&state_str).expect("failed to parse state JSON"),
        Err(_) => default_state,
    }
}
